local function read_file_lines()
  local lines = {}
  for line in io.lines() do
    lines[#lines + 1] = line
  end
  return lines
end

local function get_last_elements(lines, can_move_multiple)
  local stacks = {}
  local is_moving = false

  for _, line in ipairs(lines) do
    if not is_moving then
      if line == '' then
        is_moving = true
      else
        for match, pos in line:gmatch('%[(%u)%]()') do
          local position = pos // 4
          stacks[position] = stacks[position] or {}
          table.insert(stacks[position], 1, match)
        end
      end
    else
      local amount, from, to = line:match('move (%d+) from (%d+) to (%d+)')
      amount, from, to = tonumber(amount), tonumber(from), tonumber(to)
      local position_to = #stacks[to] + 1
      for _ = 1, amount do
        if not can_move_multiple then
          table.insert(stacks[to], table.remove(stacks[from]))
        else
          table.insert(stacks[to], position_to, table.remove(stacks[from]))
        end
      end
    end
  end

  local last_elements_of_stacks = {}

  for i = 1, #stacks do
    last_elements_of_stacks[i] = stacks[i][#stacks[i]]
  end

  return last_elements_of_stacks
end

local lines = read_file_lines()

print('Part 1', table.concat(get_last_elements(lines, false)))
print('Part 2', table.concat(get_last_elements(lines, true)))
