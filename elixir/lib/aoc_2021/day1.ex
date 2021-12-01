defmodule AoC2021.Day1 do
  def process(input) do
    Stream.map(input, &String.to_integer/1)
  end

  def part1(input) do
    input
    |> Stream.chunk_every(2, 1, :discard)
    |> Enum.count(fn [a, b] -> a < b end)
  end

  def part2(input) do
    input
    |> Stream.chunk_every(3, 1)
    |> Stream.map(&Enum.sum/1)
    |> part1()
  end
end
