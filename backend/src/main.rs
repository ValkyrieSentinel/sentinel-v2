use aya::{
    maps::{HashMap as BpfHashMap, lpm_trie::{LpmTrie, Key}},
    programs::{Xdp, XdpFlags},
    Bpf,
};
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use anyhow::Context;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let kernel_path = "../../target/bpfel-unknown-none/release/sentinel-ebpf";
    
    println!("Kernel lookup by path: {}", kernel_path);
    if !Path::new(kernel_path).exists() {
        return Err(anyhow::anyhow!(
            "Kernel not found! Please build the project first: cd ../kernel && cargo build --release --target=bpfel-unknown-none"
        ));
    }

    let mut bpf = Bpf::load_file(kernel_path)
        .context("Critical error loading eBPF file")?;

    let program: &mut Xdp = bpf.program_mut("soul_guard_v2")
        .context("Function soul_guard_v2 not found in object file")?
        .try_into()?;
    
    program.load().context("Error loading program into kernel")?;
    
    let interface = "wlo1"; 
    program.attach(interface, XdpFlags::default())
        .context(format!("Failed to attach Sentinel to {}", interface))?;

    println!("Sentinel v2 ACTIVATED on {}. Soulcode protection enabled.", interface);

    {
        let wl_map_raw = bpf.map_mut("SC_WHITELIST").context("Map SC_WHITELIST not found")?;
        let wl_map: LpmTrie<_, u32, u8> = LpmTrie::try_from(wl_map_raw)?;
        
        let dns_ip = u32::from(Ipv4Addr::new(8, 8, 8, 8)).to_be();
        let gateway_ip = u32::from(Ipv4Addr::new(192, 168, 1, 1)).to_be();
        
        let _ = wl_map.insert(&Key::new(32, dns_ip), 1, 0);
        let _ = wl_map.insert(&Key::new(32, gateway_ip), 1, 0);
    }

    let bpf_shared = Arc::new(Mutex::new(bpf));

    let bl_cleaner = bpf_shared.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(20)).await;
            let mut bpf_lock = bl_cleaner.lock().await;
            
            if let Ok(bl_map_raw) = bpf_lock.map_mut("SC_BLACKLIST") {
                if let Ok(mut bl_map) = BpfHashMap::<_, u32, u64>::try_from(bl_map_raw) {
                    let keys: Vec<u32> = bl_map
                        .iter()
                        .filter_map(|res| res.ok().map(|(k, _)| k))
                        .collect();
                    
                    for k in keys {
                        let _ = bl_map.remove(&k);
                    }
                }
            }
        }
    });

    let stats_monitor = bpf_shared.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            let bpf_lock = stats_monitor.lock().await;
            
            if let Ok(stats_map_raw) = bpf_lock.map("SC_STATS") {
                if let Ok(stats_map) = BpfHashMap::<_, u32, u64>::try_from(stats_map_raw) {
                    for res in stats_map.iter().filter_map(|e| e.ok()) {
                        let (id, val) = res;
                        if val > 0 {
                            match id {
                                100 => println!("Whitelist stream: {} packets", val),
                                200 => println!("ATTACK BLOCKED: {} attempts", val),
                                30 => println!("SYN-flood detected"),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    });

    tokio::signal::ctrl_c().await?;
    println!("\nSentinel stopped by user.");
    Ok(())
}
