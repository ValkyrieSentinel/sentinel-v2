# SENTINEL V2 

**Autonomous eBPF-based Defense Infrastructure**

Sentinel V2 is a hybrid, high-performance security platform designed for zero-latency traffic filtering and active threat mitigation. It combines the speed of kernel-level filtering with the intelligent deception of distributed honeypot systems.

###  Architecture Stack
* **Data Plane (Kernel):** eBPF/XDP written in Rust (Aya) for line-rate packet dropping.
* **Control Plane (User Space):** * **Rust:** Orchestrator & logic controller.
    * **C++:** Stateful Deep Packet Inspection (DPI) agent.
    * **Go/Elixir:** Distributed "Ghost Network" honeypots to exhaust attacker resources.
* **Observability:** Prometheus & Grafana stack for real-time attack visualization.

###  Key Capabilities
* **Kernel-level Filtering:** Immediate blocking of malicious IP addresses using `LpmTrie` and `HashMap`.
* **Stateful DPI:** Behavioral analysis and fingerprinting (TTL/TCP Window).
* **Active Deception:** Distributed traps ("Ghost Network") that force attackers into resource-heavy "conversations."
* **Auto-Adaptive:** Automated blacklist management based on real-time threat telemetry.

###  Quick Start
1. **Build the eBPF Kernel:**
   ```bash
   cd kernel
   cargo build --release --target=bpfel-unknown-none
   ```
2. **Launch the Orchestrator:**
   ```bash
   cargo run --release
   ```
3. **Deploy Traps:**
   ```bash
   go run main.go
   ```



