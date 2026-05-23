defmodule SentinelTrap.MixProject do
  use Mix.Project

  def project do
    [
      app: :sentinel_trap,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
     
      releases: [
        sentinel_trap: [
          include_executables_for: [:unix],
          applications: [runtime_tools: :permanent]
        ]
      ]
    ]
  end

 
  def application do
    [
      extra_applications: [:logger, :crypto],
      mod: {SentinelTrap.Application, []} # Вказуємо точку входу
    ]
  end

  
  defp deps do
    [
     
      {:jason, "~> 1.4"},
      
     
      {:telemetry, "~> 1.0"},
      
     
      {:timex, "~> 3.7"}
    ]
  end
end