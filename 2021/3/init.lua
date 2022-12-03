local lines = {}
local generator = {}
local scrubber = {}

local function count_digits(lns)
  local counts = {}
  for _, line in ipairs(lns) do
    for i = 1, #line do
      if not counts[i] then counts[i] = 0 end
      local char = line:sub(i, i)
      if (char == '1') then
        counts[i] = counts[i] + 1
      else
        counts[i] = counts[i] - 1
      end
    end
  end
  return counts
end

for line in io.lines() do
  table.insert(lines, line)
  table.insert(generator, line)
  table.insert(scrubber, line)
end

local counts = count_digits(lines)
local gamma = {}
local epsilon = {}

for i = 1, #counts do
  local sum = counts[i]
  table.insert(gamma, sum > 0 and '1' or '0')
  table.insert(epsilon, sum > 0 and '0' or '1')
end

local function bit_criteria(list, if_one, if_zero)
  local char_index = 1
  while #list ~= 1 do
    local c = count_digits(list)
    local char = c[char_index] >= 0 and if_one or if_zero
    local i = 1
    while list[i] ~= nil do
      local line = list[i]
      if (char == line:sub(char_index, char_index)) then
        i = i + 1
      else
        table.remove(list, i)
      end
    end
    char_index = char_index + 1
  end
end

bit_criteria(generator, '1', '0')
bit_criteria(scrubber, '0', '1')

print('Part 1:', tonumber(table.concat(gamma), 2) * tonumber(table.concat(epsilon), 2))
print('Part 2:', tonumber(generator[1], 2) * tonumber(scrubber[1], 2))
