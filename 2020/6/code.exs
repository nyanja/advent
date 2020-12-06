defmodule Advent do
  def s1([]), do: 0
  def s1 [group | rest] do
    group
    |> String.to_charlist
    |> Enum.uniq
    |> Kernel.--('\n')
    |> length
    |> Kernel.+(s1(rest))
  end


  defp collect([item]), do: MapSet.new(String.to_charlist(item))
  defp collect [item | rest] do
    item
    |> String.to_charlist
    |> MapSet.new
    |> MapSet.intersection(collect(rest))
  end

  def s2([]), do: 0
  def s2 [group | rest] do
    group
    |> String.split("\n")
    |> collect
    |> MapSet.to_list
    |> length
    |> Kernel.+(s2(rest))
  end
end

input = File.read!("input") |> String.split(~r/\n\n/)

IO.puts Advent.s1 input
IO.puts Advent.s2 input
