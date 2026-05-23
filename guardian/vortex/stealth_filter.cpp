#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>
#include <regex>
#include <netinet/ip.h>
#include <netinet/tcp.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <ctime>
#include <bpf/libbpf.h>
#include <bpf/bpf.h>

#define PCAP_DONT_INCLUDE_PCAP_BPF_H
#include <pcap.h>

struct bpf_program {
    u_int bf_len;
    struct bpf_insn *bf_insns;
};

const char* BPF_MAP_PIN_PATH = "/sys/fs/bpf/sentinel/blacklist_map";

struct Session {
    std::string stream;
    time_t last_activity;
    uint8_t last_ttl;
    uint16_t last_win;
    std::string os_fingerprint;
};

std::map<std::string, Session> session_pool;

const std::vector<std::string> SIGNATURES = {
    "union select", "drop table", "select from", "<script>", 
    "etc/passwd", "or 1=1", "base64_decode", "exec(", "eval(",
    "../", "boot.ini", "cmd.exe"
};

std::string identify_stack(uint8_t ttl, uint16_t win) {
    if (ttl <= 64 && win <= 5840) return "LINUX/KALI (High Risk)";
    if (ttl <= 128 && win > 8192) return "WINDOWS (Standard)";
    return "UNKNOWN/PROXY";
}

std::string normalize_payload(std::string data) {
    std::transform(data.begin(), data.end(), data.begin(), ::tolower);
    std::regex noise_regex("/\\*.*?\\*/|--.*|[\\s\\t\\n\\r]+");
    return std::regex_replace(data, noise_regex, " ");
}

void kernel_ban_ip(uint32_t ip_addr_be) {
    int fd = bpf_obj_get(BPF_MAP_PIN_PATH);
    if (fd < 0) return;

    uint64_t expiry = 1; 
    bpf_map_update_elem(fd, &ip_addr_be, &expiry, BPF_ANY);
    close(fd);
}

void packet_handler(u_char *user_data, const struct pcap_pkthdr *pkthdr, const u_char *packet) {
    struct ip *ip_hdr = (struct ip *)(packet + 14);
    
    if (ip_hdr->ip_p == IPPROTO_TCP) {
        int ip_hl = ip_hdr->ip_hl * 4;
        struct tcphdr *tcp_hdr = (struct tcphdr *)(packet + 14 + ip_hl);
        int tcp_off = tcp_hdr->doff * 4;
        
        const char *payload = (const char *)(packet + 14 + ip_hl + tcp_off);
        int payload_len = pkthdr->len - (14 + ip_hl + tcp_off);

        if (payload_len > 0) {
            char src_ip[INET_ADDRSTRLEN];
            inet_ntop(AF_INET, &(ip_hdr->ip_src), src_ip, INET_ADDRSTRLEN);

            std::string session_id = std::string(src_ip) + ":" + std::to_string(ntohs(tcp_hdr->source));
            auto& s = session_pool[session_id];
            
            s.stream.append(payload, payload_len);
            s.last_activity = time(NULL);
            s.last_ttl = ip_hdr->ip_ttl;
            s.last_win = ntohs(tcp_hdr->window);
            s.os_fingerprint = identify_stack(s.last_ttl, s.last_win);

            std::string analyzed_data = normalize_payload(s.stream);
            for (const auto& sig : SIGNATURES) {
                if (analyzed_data.find(sig) != std::string::npos) {
                    std::cout << "[SENTINEL] BANNING ATTACKER: " << src_ip << " | INFO: " << s.os_fingerprint << std::endl;
                    kernel_ban_ip(ip_hdr->ip_src.s_addr);
                    session_pool.erase(session_id);
                    break;
                }
            }
        }
    }
}

int main(int argc, char *argv[]) {
    char errbuf[PCAP_ERRBUF_SIZE];
    std::string device = (argc > 1) ? argv[1] : "eth0"; 

    pcap_t *handle = pcap_open_live(device.c_str(), BUFSIZ, 1, 10, errbuf);
    if (!handle) {
        std::cerr << "Device error: " << errbuf << std::endl;
        return 2;
    }

    struct bpf_program fp;
    std::string filter_exp = "tcp and (dst port 80 or dst port 443 or dst port 4000)";
    
    if (pcap_compile(handle, (struct bpf_program*)&fp, filter_exp.c_str(), 0, PCAP_NETMASK_UNKNOWN) == -1) {
        std::cerr << "Filter error" << std::endl;
        return 2;
    }
    pcap_setfilter(handle, (struct bpf_program*)&fp);

    std::cout << "Helga's Sentinel DPI active on " << device << "..." << std::endl;

    pcap_loop(handle, 0, packet_handler, NULL);
    pcap_close(handle);
    return 0;
}