defmodule Advent do
  defp keys_present?(entry) do
    if length(Regex.scan(~r/...:/, entry) -- [["cid:"]]) == 7, do: 1, else: 0
  end

  def s1([]), do: 0
  def s1([passport | rest]), do: keys_present?(passport) + s1(rest)


  defp valid_passport?(entry) do
    r = ~r/(byr):(\d{4})\s|(iyr):(\d{4})\s|(eyr):(\d{4})\s|(hgt):(\d{2,3})\w\w\s|(hcl):(#[0-9a-f]{6})\s|(ecl):(amb|blu|brn|gry|grn|hzl|oth)\s|(pid):(\d{9})\s/

    m = Regex.scan(r, entry <> " ")
          |> Enum.map(fn i -> {Enum.at(i, -2), Enum.at(i, -1)} end)
          |> Map.new

    if m["byr"] && String.to_integer(m["byr"]) in 1920..2002 &&
       m["iyr"] && String.to_integer(m["iyr"]) in 2010..2020 &&
       m["eyr"] && String.to_integer(m["eyr"]) in 2020..2030 &&
       m["hgt"] && (Regex.match?(~r/cm/, entry) && String.to_integer(m["hgt"]) in 150..193 ||
                    Regex.match?(~r/in/, entry) && String.to_integer(m["hgt"]) in 59..76) &&
       m["hcl"] && m["ecl"] && m["pid"] do
      1
    else
      0
    end
  end

  def s2([]), do: 0
  def s2([passport | rest]), do: valid_passport?(passport) + s2(rest)
end

input = File.read!("input") |> String.split(~r/\n\n/)

IO.puts(Advent.s1(input))
IO.puts(Advent.s2(input))
