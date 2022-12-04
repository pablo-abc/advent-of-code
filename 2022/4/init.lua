local contained_count = 0
local overlap_count = 0

local function contains_point(point, start, finish)
  return point >= start and point <= finish
end

for line in io.lines() do
  local ranges = table.pack(line:match('(%d+)-(%d+),(%d+)-(%d+)'))
  local first_start = tonumber(ranges[1])
  local first_end = tonumber(ranges[2])
  local second_start = tonumber(ranges[3])
  local second_end = tonumber(ranges[4])

  -- Part 1
  local one_contains_the_other =
  (first_start <= second_start and first_end >= second_end) or
      (second_start <= first_start and second_end >= first_end)
  if (one_contains_the_other) then contained_count = contained_count + 1 end

  -- Part 2
  local is_overlapping = contains_point(first_start, second_start, second_end)
      or contains_point(first_end, second_start, second_end)
      or contains_point(second_start, first_start, first_end)
      or contains_point(second_end, first_start, first_end)

  if (is_overlapping) then overlap_count = overlap_count + 1 end
end

print('Part 1:', contained_count)
print('Part 2:', overlap_count)
