local rucksack_sum = 0

local function get_priority(char)
  local is_lower_case = char:match('[a-z]')
  if (is_lower_case) then
    return char:byte() - 96
  else
    return char:byte() - 38
  end
end

local groups = { {} }
local badge_sum = 0

local line_index = 1
for line in io.lines() do
  -- Part 1
  local found_comp_chars = {}
  local split_pos = #line / 2
  local first_compartment = line:sub(1, split_pos)
  local second_compartment = line:sub(split_pos + 1)

  for i = 1, split_pos do
    local char = first_compartment:sub(i, i)
    local is_in_both = second_compartment:find(char)
    if is_in_both and not found_comp_chars[char] then
      local priority = get_priority(char)
      rucksack_sum = rucksack_sum + priority
    end
    found_comp_chars[char] = true
  end

  -- Part 2
  local found_badge_chars = {}
  table.insert(groups[#groups], line)
  if (line_index % 3 == 0) then
    local group = groups[#groups]
    for i = 1, #group[1] do
      local char = group[1]:sub(i, i)
      local is_in_all = group[2]:find(char) and group[3]:find(char)
      if is_in_all and not found_badge_chars[char] then
        badge_sum = badge_sum + get_priority(char)
      end
      found_badge_chars[char] = true
    end
    groups[#groups + 1] = {}
  end
  line_index = line_index + 1
end


print('Part 1:', rucksack_sum)
print('Part 2:', badge_sum)
