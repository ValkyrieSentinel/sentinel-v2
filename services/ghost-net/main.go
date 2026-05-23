package main

import (
    "fmt"
    "net"
    "net/http"
    "strings"
    "time"

    "github.com/prometheus/client_golang/prometheus"
    "github.com/prometheus/client_golang/prometheus/promauto"
    "github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
    botsTrapped = promauto.NewCounterVec(prometheus.CounterOpts{
        Name: "sentinel_ghost_net_trapped_total",
        Help: "Total number of bots trapped in musical playlists",
    }, []string{"genre", "port"})

    activeConnections = promauto.NewGauge(prometheus.GaugeOpts{
        Name: "sentinel_ghost_net_active_listeners",
        Help: "Current number of bots currently listening to music",
    })
)

var playlists = map[string][]string{
    "nirvana": {
        "Load up on guns, bring your friends",
        "It's fun to lose and to pretend",
        "She's over-bored and self-assured",
        "Oh no, I know a dirty word",
        "Hello, hello, hello, how low?",
    },
    "aerosmith": {
        "I'm dreamin' until my dreams come true",
        "Dream on, dream on, dream on",
        "Dream until your dreams come true",
        "Sing with me, sing for the years",
        "Sing for the laughter, sing for the tears",
    },
}

func startTrap(conn net.Conn, genre string, port int) {
    defer conn.Close()
    activeConnections.Inc()
    defer activeConnections.Dec()

    remoteAddr := conn.RemoteAddr().String()
    portStr := fmt.Sprintf("%d", port)
    
    fmt.Printf("[] Intrusion detected! Bot %s trapped on port %d (%s)\n", remoteAddr, port, strings.ToUpper(genre))

    botsTrapped.WithLabelValues(genre, portStr).Inc()

    conn.Write([]byte("HTTP/1.1 200 OK\r\n"))
    conn.Write([]byte("Server: Sentinel-v2-Valkyrie/Kernelska\r\n"))
    conn.Write([]byte("Content-Type: text/plain; charset=UTF-8\r\n"))
    conn.Write([]byte("Connection: keep-alive\r\n\r\n"))
    
    conn.Write([]byte(fmt.Sprintf("Welcome to the Ghost Network. (c) Natalia Kernelska\n")))
    conn.Write([]byte(fmt.Sprintf("Current mood: %s\n", strings.ToUpper(genre))))
    conn.Write([]byte("--------------------------------------------------\n\n"))

    for {
        for _, line := range playlists[genre] {
            _, err := conn.Write([]byte("🎵 " + line + "\n"))
            if err != nil {
                fmt.Printf("[] Bot %s disconnected (could not withstand the rock).\n", remoteAddr)
                return
            }
            time.Sleep(1800 * time.Millisecond)
        }
    }
}

func main() {
    fmt.Println("###################################################")
    fmt.Println("#   SENTINEL v2: GHOST NETWORK ACTIVE          #")
    fmt.Println("#  Architect: Natalia Kernelska (c)             #")
    fmt.Println("# ⚡ Status: Monitoring ports 1000-1050           #")
    fmt.Println("###################################################")

    go func() {
        fmt.Println("[] Metrics server starting at :2112/metrics")
        http.Handle("/metrics", promhttp.Handler())
        if err := http.ListenAndServe(":2112", nil); err != nil {
            fmt.Printf("Metrics error: %v\n", err)
        }
    }()

    for port := 1000; port <= 1050; port++ {
        go func(p int) {
            addr := fmt.Sprintf(":%d", p)
            listener, err := net.Listen("tcp", addr)
            if err != nil {
                return 
            }
            
            genre := "nirvana"
            if p%2 != 0 {
                genre = "aerosmith"
            }

            for {
                conn, err := listener.Accept()
                if err != nil {
                    continue
                }
                go startTrap(conn, genre, p)
            }
        }(port)
    }

    fmt.Println(" Ghost Net is ready. Waiting for bots...")
    
    select {}
}