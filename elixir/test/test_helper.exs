defmodule AoC2021 do
  def read_input(day) do
    "../inputs/#{day}.txt"
    |> File.stream!()
    |> Stream.map(&String.trim/1)
  end
end

ExUnit.start()
