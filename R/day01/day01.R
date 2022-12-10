library(tidyverse)
input = readLines(here::here("R/day01/input"))
input_tbl = as_tibble_col(input, column_name = "calories")
# Credit to Andrea Gilardi for use of cumsum:
# https://github.com/agila5/aoc2022/blob/main/R/day01.R
elf_calories = input_tbl %>%
  mutate(elf_id = cumsum(input == "") + 1) %>%
  group_by(elf_id) %>%
  summarise(calories = sum(as.numeric(calories), na.rm = TRUE))
elf_calories %>%
  slice(which.max(calories))
elf_calories %>%
  arrange(desc(calories)) %>%
  head(3)
