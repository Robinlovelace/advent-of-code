# Part 1
input <- readLines(here::here("R/day01/input"))
idx_elf <- cumsum(input == "")
by(input, factor(idx_elf), \(x) sum(as.numeric(x), na.rm = TRUE)) |> max()

# Part 2
input <- readLines(here::here("R/day01/input"))
idx_elf <- cumsum(input == "")
by(input, factor(idx_elf), \(x) sum(as.numeric(x), na.rm = TRUE)) |>
  sort(decreasing = TRUE) |>
  head(3) |>
  sum()
