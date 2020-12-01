defmodule Advent do

  # find the two entries that sum to 2020 and then multiply those two numbers together
  def s1(input) do
    x = Enum.find(input, fn i -> Enum.member?(input, 2020 - i) end)
    x * (2020 - x)
  end

  # what is the product of the three entries that sum to 2020?
  def s2(input, x, y) do
    {l, m, r} = {Enum.at(input, x), Enum.at(input, y), Enum.at(input, -1)}
    sum = l + m + r

    cond do
      sum > 2020 ->
        if y == (length(input) - 2) do
          s2(input -- [r], 0, 1)
        else
          s2(input, 0, y + 1)
        end
      sum < 2020 ->
        if (y - x) > 1 do
          s2(input, x + 1, y)
        else
          s2(input, 0, y + 1)
        end
      true -> sum
    end
  end
end

input = ~w/#{File.read!("input")}/ |> Enum.map(&String.to_integer/1)

IO.puts(Advent.s1(input))
IO.puts(Advent.s2(Enum.sort(input), 0, 1))
