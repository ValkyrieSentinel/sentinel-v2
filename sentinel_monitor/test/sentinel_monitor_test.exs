defmodule SentinelMonitorTest do
  use ExUnit.Case
  doctest SentinelMonitor

  test "greets the world" do
    assert SentinelMonitor.hello() == :world
  end
end
