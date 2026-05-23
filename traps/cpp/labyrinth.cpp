#include <iostream>
#include <string>
#include <vector>
#include <thread>
#include <netinet/in.h>
#include <arpa/inet.h> 
#include <unistd.h>
#include <chrono>

class SentinelLabyrinth {
private:
    int server_fd;
    int port;

    void handle_bot(int client_socket, std::string client_ip) {
        std::cout << "[!] Target Locked: " << client_ip << " entering the labyrinth...\n";
        
        std::string welcome = "Welcome to SecureStorage v2.0\nType 'ls' to see files or 'cd' to navigate.\n";
        send(client_socket, welcome.c_str(), welcome.length(), 0);

        std::vector<std::string> fake_dirs = {"etc", "var", "secret_data", "backup_2024", "private_keys"};
        
        while (true) {
            char buffer[1024] = {0};
            int valread = read(client_socket, buffer, 1024);
            if (valread <= 0) break;

            std::this_thread::sleep_for(std::chrono::milliseconds(800));

            std::string response = "Entering subdirectory: " + fake_dirs[rand() % 5] + "/root/node_" + std::to_string(rand() % 1000) + "...\n";
            if (send(client_socket, response.c_str(), response.length(), 0) <= 0) break;
        }
        close(client_socket);
        std::cout << "[x] Labyrinth collapsed for: " << client_ip << "\n";
    }

public:
    SentinelLabyrinth(int p) : port(p) {
        server_fd = socket(AF_INET, SOCK_STREAM, 0);
        int opt = 1;
        setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

        sockaddr_in address;
        address.sin_family = AF_INET;
        address.sin_addr.s_addr = INADDR_ANY;
        address.sin_port = htons(port);

        bind(server_fd, (struct sockaddr*)&address, sizeof(address));
        listen(server_fd, 100);
    }

    void start() {
        std::cout << " C++ Sentinel Labyrinth Core started on port " << port << "\n";
        while (true) {
            sockaddr_in client_addr;
            socklen_t addrlen = sizeof(client_addr);
            int client_socket = accept(server_fd, (struct sockaddr*)&client_addr, &addrlen);

            if (client_socket >= 0) {
                
                std::string ip = inet_ntoa(client_addr.sin_addr);
                std::thread(&SentinelLabyrinth::handle_bot, this, client_socket, ip).detach();
            }
        }
    }
};

int main() {
    srand(time(0));
    SentinelLabyrinth trap(9999);
    trap.start();
    return 0;
}