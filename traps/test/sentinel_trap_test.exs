defmodule SentinelTrapTest do
  use ExUnit.Case
  doctest SentinelTrap

  test "greets the world" do
    assert SentinelTrap.hello() == :world
  end
end
