defmodule AoC2021.Day2 do
  @type submarine :: %{horizontal: integer(), depth: integer(), aim: integer()}
  @type command :: %{direction: :forward | :up | :down, distance: integer()}

  @spec read_command(line :: String.t()) :: command()
  defp read_command(line) do
    [direction, distance] = String.split(line)
    %{direction: String.to_atom(direction), distance: String.to_integer(distance)}
  end

  @spec process(lines :: Enumerable.t()) :: commands :: Enumerable.t()
  def process(lines) do
    Stream.map(lines, &read_command/1)
  end

  def part1(commands) do
    %{horizontal: horizontal, depth: depth} =
      Enum.reduce(commands, %{horizontal: 0, depth: 0}, fn
        %{direction: :forward, distance: distance}, %{horizontal: horizontal} = sub ->
          %{sub | horizontal: horizontal + distance}

        %{direction: :up, distance: distance}, %{depth: depth} = sub ->
          %{sub | depth: depth - distance}

        %{direction: :down, distance: distance}, %{depth: depth} = sub ->
          %{sub | depth: depth + distance}
      end)

    horizontal * depth
  end

  def part2(commands) do
    %{horizontal: horizontal, depth: depth} =
      Enum.reduce(commands, %{horizontal: 0, depth: 0, aim: 0}, fn
        %{direction: :forward, distance: distance},
        %{horizontal: horizontal, depth: depth, aim: aim} = sub ->
          %{sub | horizontal: horizontal + distance, depth: depth + aim * distance}

        %{direction: :up, distance: distance}, %{aim: aim} = sub ->
          %{sub | aim: aim - distance}

        %{direction: :down, distance: distance}, %{aim: aim} = sub ->
          %{sub | aim: aim + distance}
      end)

    horizontal * depth
  end
end
