local found_sop_markers = {}
local found_som_markers = {}
local lines = {}

for line in io.lines() do
  table.insert(lines, line)
  local last_four = {}
  local last_fourteen = {}
  for char_pos = 1, #line do
    local char = line:sub(char_pos, char_pos)

    table.insert(last_four, char)
    table.insert(last_fourteen, char)

    if #last_four > 4 then
      table.remove(last_four, 1)
    end

    if #last_fourteen > 14 then
      table.remove(last_fourteen, 1)
    end

    if #last_four == 4 and #found_sop_markers < #lines then
      local hash = {}
      for i = 1, #last_four do
        if hash[last_four[i]] then
          break
        end
        if (i == #last_four) then
          table.insert(found_sop_markers, char_pos)
        end
        hash[last_four[i]] = true
      end
    end

    if #last_fourteen == 14 then
      local hash = {}
      for i = 1, #last_fourteen do
        if hash[last_fourteen[i]] then
          break
        end
        if (i == #last_fourteen) then
          table.insert(found_som_markers, char_pos)
        end
        hash[last_fourteen[i]] = true
      end
    end
    if #found_som_markers == #lines then
      break
    end
  end
end

print('Part 1:', table.concat(found_sop_markers, ", "))
print('Part 2:', table.concat(found_som_markers, ", "))
