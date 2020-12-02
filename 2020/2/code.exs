defmodule Advent do

  # Each line gives the password policy and then the password.
  # The password policy indicates the lowest and highest number
  # of times a given letter must appear for the password to be valid.
  # For example, 1-3 a means that the password must contain a at least
  # 1 time and at most 3 times.

  # How many passwords are valid according to their policies?

  def s1([]), do: 0

  def s1([min, max, char, s | rest]) do
    l = Regex.scan(~r/#{char}/, s) |> length
    if l >= String.to_integer(min) && l <= String.to_integer(max) do
      s1(rest) + 1
    else
      s1(rest)
    end
  end


  # Each policy actually describes two positions in the password,
  # where 1 means the first character, 2 means the second character,
  # and so on. (Be careful; Toboggan Corporate Policies have no concept
  # of "index zero"!) Exactly one of these positions must contain the
  # given letter. Other occurrences of the letter are irrelevant
  # for the purposes of policy enforcement.

  def s2([]), do: 0

  def s2([min, max, char, s | rest]) do
    if (char == String.at(s, String.to_integer(min) - 1)) !=
       (char == String.at(s, String.to_integer(max) - 1)) do
      s2(rest) + 1
    else
      s2(rest)
    end
  end
end


input = Regex.split(~r{-| |: |\n}, File.read!("input"))

IO.puts(Advent.s1(input))
IO.puts(Advent.s2(input))
