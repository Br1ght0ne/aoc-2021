defmodule AoC2021.Day1Test do
  use ExUnit.Case
  alias AoC2021.Day1

  def input() do
    Day1.process(AoC2021.read_input(1))
  end

  test "part 1" do
    assert Day1.part1(input()) == 1154
  end

  test "part 2" do
    assert Day1.part2(input()) == 1127
  end
end
