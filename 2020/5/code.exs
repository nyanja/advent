defmodule Advent do

  defp seat_id pass do
    [{row, _}, {col, _}] = String.replace(pass, ["B", "R"], "1")
                           |> String.replace(["F", "L"], "0")
                           |> String.split_at(7)
                           |> Tuple.to_list
                           |> Enum.map(&Integer.parse(&1, 2))
    row * 8 + col
  end

  def s1([]), do: 0
  def s1([pass | rest]), do: max seat_id(pass), s1(rest)

  def s2 input do
    seats = Enum.map(input, &seat_id/1)
    [x] = Enum.to_list(Enum.min(seats) .. Enum.max(seats)) -- seats
    x
  end
end

input = File.read!("input") |> String.split

IO.puts Advent.s1 input
IO.puts Advent.s2 input
