#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{HashMap, LpmTrie, lpm_trie::Key},
    programs::XdpContext,
};
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, IpProto},
    udp::UdpHdr,
    tcp::TcpHdr,
};


#[map]
static SC_STATS: HashMap<u32, u64> = HashMap::<u32, u64>::with_max_entries(128, 0);

#[map]
static SC_WHITELIST: LpmTrie<u32, u8> = LpmTrie::<u32, u8>::with_max_entries(4096, 0);

#[map]
static SC_BLACKLIST: HashMap<u32, u64> = HashMap::<u32, u64>::with_max_entries(16384, 0);


#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    if start + offset + mem::size_of::<T>() > end {
        return Err(());
    }
    Ok((start + offset) as *const T)
}

#[inline(always)]
fn update_stat(id: u32) {
    unsafe {
        let val = SC_STATS.get(&id).copied().unwrap_or(0);
        let _ = SC_STATS.insert(&id, &(val + 1), 0);
    }
}

#[inline(always)]
fn get_l4_offset(ipv4hdr: *const Ipv4Hdr) -> usize {
    let ihl = unsafe { (*ipv4hdr).ihl() } as usize;
    EthHdr::LEN + (ihl * 4)
}


#[xdp]
pub fn soul_guard_v2(ctx: XdpContext) -> u32 {
    match try_soul_guard(&ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
fn try_soul_guard(ctx: &XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(ctx, 0)? };
    let eth_type = unsafe { (*ethhdr).ether_type };

    
    if eth_type == EtherType::Arp {
        return Ok(xdp_action::XDP_PASS);
    }
    if eth_type != EtherType::Ipv4 {
        return Ok(xdp_action::XDP_PASS);
    }

    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(ctx, EthHdr::LEN)? };
    let src_ip = unsafe { (*ipv4hdr).src_addr };
    let src_host = u32::from_be(src_ip);

    
    if (src_host >> 16) == 0xC0A8 || (src_host >> 24) == 127 || (src_host >> 24) == 10 {
        return Ok(xdp_action::XDP_PASS);
    }

    
    let wl_key = Key::new(32, src_ip);
    if SC_WHITELIST.get(&wl_key).is_some() {
        update_stat(100);
        return Ok(xdp_action::XDP_PASS);
    }

    unsafe {
        if SC_BLACKLIST.get(&src_ip).is_some() {
            update_stat(200);
            return Ok(xdp_action::XDP_DROP);
        }
    }

    
    let l4_offset = get_l4_offset(ipv4hdr);
    let proto = unsafe { (*ipv4hdr).proto };

    match proto {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, l4_offset)? };
            let d_port = u16::from_be(unsafe { (*tcphdr).dest });

            
            if d_port == 5173 || d_port == 3000 || d_port == 8080 {
                update_stat(3000);
                return Ok(xdp_action::XDP_PASS);
            }

            
            let is_syn = unsafe { (*tcphdr).syn() != 0 && (*tcphdr).ack() == 0 };
            if is_syn {
                update_stat(30);
            }
        }
        IpProto::Udp => {
            let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, l4_offset)? };
            let d_port = u16::from_be(unsafe { (*udphdr).dest });

            
            if d_port == 67 || d_port == 68 || d_port == 53 {
                return Ok(xdp_action::XDP_PASS);
            }

            
            if unsafe { u16::from_be((*ipv4hdr).tot_len) } > 1450 {
                update_stat(400);
                return Ok(xdp_action::XDP_DROP);
            }
        }
        IpProto::Icmp => {
            update_stat(1);
        }
        _ => {
            update_stat(999);
        }
    }

    update_stat(777); 
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}