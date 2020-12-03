defmodule Advent do

  def s1(input), do: s1(input, 0, 0, 3, 1)

  def s1([], _, _, _, _), do: 0

  def s1([s | rest], x, y, right, down) do
    if y == 0 do
      i = if String.at(s, x) == "#", do: 1, else: 0
      i + s1(rest,
             Integer.mod(x + right, String.length(s)),
             Integer.mod(y + 1, down),
             right,
             down)
    else
      s1(rest, x, Integer.mod(y + 1, down), right, down)
    end
  end


  def s2(input) do
    Enum.reduce([[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]], 1, fn [right, down], acc ->
      acc * Advent.s1(input, 0, 0, right, down)
    end)
  end
end


input = File.stream!("input") |> Stream.map(&String.trim/1) |> Enum.to_list

IO.puts(Advent.s1(input))
IO.puts(Advent.s2(input))
