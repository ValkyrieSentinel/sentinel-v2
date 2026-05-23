defmodule Sentinel.Trap do
  require Logger

  def start(port \\ 4000) do
    {:ok, listen_socket} = :gen_tcp.listen(port, [
      :binary, 
      packet: :line, 
      active: false, 
      reuseaddr: true
    ])
    Logger.info("⚡ Elixir Sentinel Trap active on port #{port}. The game has started.")
    loop_acceptor(listen_socket)
  end

  defp loop_acceptor(listen_socket) do
    {:ok, client_socket} = :gen_tcp.accept(listen_socket)
    spawn(fn -> handle_bot(client_socket) end)
    loop_acceptor(listen_socket)
  end

  defp handle_bot(socket) do
    case :gen_tcp.recv(socket, 0) do
      {:ok, data} ->
        payload = String.trim(data)
        
        strategy = analyze_threat(payload)

        attack_report = %{
          event: "bot_capture",
          payload: payload,
          detected_strategy: strategy,
          timestamp: DateTime.utc_now() |> DateTime.to_iso8601(),
          module: "Elixir.Sentinel.Trap",
          architect: "Helga"
        }

        log_json_report(attack_report)

        respond_with_bluff(socket, payload, strategy)
        
        handle_bot(socket)

      {:error, :closed} ->
        Logger.info("🛡️ The mafia has left the table.")
      {:error, reason} ->
        Logger.error(" Connection error: #{inspect(reason)}")
    end
  end

  defp analyze_threat(payload) do
    cond do
      byte_size(payload) > 1024 -> :deep_sleep
      String.contains?(payload, "select") -> :fake_database
      String.contains?(payload, "admin")  -> :fake_login
      true                                -> :echo_delay
    end
  end

  defp respond_with_bluff(socket, _data, :fake_database) do
    :timer.sleep(500)
    fake_hash = :crypto.strong_rand_bytes(16) |> Base.encode16()
    response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n" <>
               "[{\"id\":1, \"user\":\"admin\", \"pass\":\"$2y$10$#{fake_hash}\"}]"
    :gen_tcp.send(socket, response)
  end

  defp respond_with_bluff(socket, _data, :fake_login) do
    :gen_tcp.send(socket, "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"Restricted\"\r\n\r\n")
  end

  defp respond_with_bluff(socket, _data, :deep_sleep) do
    Logger.warning(" Putting bot into deep sleep (Large payload)")
    :timer.sleep(30_000)
    :gen_tcp.send(socket, "HTTP/1.1 408 Request Timeout\r\n\r\n")
  end

  defp respond_with_bluff(socket, data, _) do
    :timer.sleep(1000)
    :gen_tcp.send(socket, "Echo from Sentinel: #{data}\n")
  end

  defp log_json_report(report) do
    case Jason.encode(report) do
      {:ok, json} -> Logger.warning(" Attack Report: #{json}")
      {:error, _} -> Logger.error("Failed to encode JSON report")
    end
  end
end