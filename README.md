# Sentinel v2: Polyglot Security Research Framework

A comprehensive, multi-language security framework combining eBPF/XDP kernel filtering, deep packet inspection (DPI), and distributed honeypot networks.

## Overview
Sentinel v2 is an experimental security research platform that demonstrates defense-in-depth strategies using a polyglot architecture. Each component is written in the language best suited for its specific task.

## Architecture
- Kernel Layer (Rust + eBPF/XDP): High-performance packet filtering, LPM Trie IP lists, SYN-flood detection.
- DPI Layer (C++ + libbpf): Deep packet inspection, signature-based attack detection, OS fingerprinting.
- Honeypot Layer (Multi-Language): Python (fake audio streams), Go (Prometheus metrics traps), C++ (fake directory labyrinth), Elixir (JSON-logging traps).
- Monitoring Layer: Prometheus metrics, real-time eBPF stats, automated reporting.

## Why Polyglot?
- Rust: Memory-safe kernel programming.
- C++: High-performance DPI with pcap.
- Go: Concurrent honeypots with metrics.
- Elixir: Fault-tolerant trap monitoring.

## Build and Run
1. Prerequisites: Rust nightly, cargo, GCC, Go, Elixir, Python 3, libbpf, pcap-dev.
2. Build eBPF: cargo build --release --target=bpfel-unknown-none
3. Build User-space: cargo build --release, go build, g++ -o dpi dpi.cpp
4. Run: sudo ./target/release/sentinel-v2

## Security Notice
Intended strictly for authorized security research and educational purposes. The author assumes no liability for misuse.

## License
MIT. Part of the ValkyrieSentinel ecosystem.
