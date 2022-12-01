local totals = { 0 }

for line in io.lines() do
  if line == '' then
    totals[#totals + 1] = 0
  else
    local c = tonumber(line)
    totals[#totals] = c + totals[#totals]
  end
end

table.sort(totals, function(a, b) return a > b end)

print('Part 1:', totals[1])
print('Part 2:', totals[1] + totals[2] + totals[3])
