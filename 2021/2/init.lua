local position = {
  forward = 0,
  down = 0,
  up = 0,
  aim = 0,
  depth = 0,
}
for line in io.lines() do
  local command = line:sub(line:find('[a-z]+'))
  local amount = line:sub(line:find('[0-9]+'))
  position[command] = position[command] + amount
  if command == 'up' then
    position.aim = position.aim - amount
  elseif command == 'down' then
    position.aim = position.aim + amount
  elseif command == 'forward' then
    position.depth = position.depth + (position.aim * amount)
  end
end

print('Part 1', position.forward * (position.down - position.up))
print('Part 2', position.forward * position.depth)
