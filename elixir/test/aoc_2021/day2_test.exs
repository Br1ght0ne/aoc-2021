defmodule AoC2021.Day2Test do
  use ExUnit.Case
  alias AoC2021.Day2

  describe "example input" do
    def example_input() do
      Day2.process(["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"])
    end

    test "part 1" do
      assert Day2.part1(example_input()) == 150
    end

    test "part 2" do
      assert Day2.part2(example_input()) == 900
    end
  end

  def input() do
    Day2.process(AoC2021.read_input(2))
  end

  test "part 1" do
    assert Day2.part1(input()) == 1_635_930
  end

  test "part 2" do
    assert Day2.part2(input()) == 1_781_819_478
  end
end
