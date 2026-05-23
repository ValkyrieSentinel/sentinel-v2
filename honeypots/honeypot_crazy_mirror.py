import socket
import threading
import time
import random
import os

PORT = 8080 

LYRICS = [
    "Come here, baby... You drive me up the wall!",
    "Nasty tricks you pull, eh? It's all a show!",
    "Girl, you got to change your crazy ways...",
    "I know you ain't wearin' nothing underneath that overcoat!",
    "I go crazy, crazy, crazy for you baby!"
]

def handle_intruder(conn, addr):
    print(f"[!] Intruder detected: {addr[0]}")
    try:
        header = (
            "HTTP/1.1 200 OK\r\n"
            "Content-Type: audio/mpeg\r\n"
            "Server: Sentinel-Stealth-Radio/1.0\r\n"
            "Cache-Control: no-cache\r\n"
            "Ice-Name: Aerosmith Tribute Radio\r\n"
            "Connection: close\r\n\r\n"
        )
        conn.send(header.encode('utf-8'))
        
        while True:
            if random.random() > 0.7:
                payload = f"--- {random.choice(LYRICS)} --- \x00\xff\xfe\x00"
            else:
                payload = os.urandom(512).decode('latin-1', errors='ignore')
            
            conn.send(payload.encode('latin-1'))
            time.sleep(random.uniform(0.5, 1.5))
            
    except (ConnectionResetError, BrokenPipeError):
        print(f"[💀] Connection closed by: {addr[0]}")
    except Exception as e:
        print(f"[?] Trap error: {e}")
    finally:
        conn.close()

def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    
    try:
        s.bind(('0.0.0.0', PORT))
        s.listen(100)
        print(f"🔊 'Crazy Radio' honeypot activated on port {PORT}")
        
        while True:
            conn, addr = s.accept()
            conn.settimeout(60)
            threading.Thread(target=handle_intruder, args=(conn, addr), daemon=True).start()
    except KeyboardInterrupt:
        print("\n[!] Sentinel Traps: Stopping...")
    finally:
        s.close()

if __name__ == "__main__":
    main()