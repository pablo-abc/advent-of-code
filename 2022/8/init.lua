local grid = {}


local function is_visible(x, y)
  if x == 1 or y == 1 or x == #grid[1] or y == #grid then
    return true
  end
  local visible = {
    top = true,
    bottom = true,
    left = true,
    right = true,
  }
  for dx = 1, #grid[1] do
    if grid[y][dx] >= grid[y][x] then
      if dx < x then
        visible.left = false
      elseif dx > x then
        visible.right = false
      end
    end
  end

  for dy = 1, #grid do
    if grid[dy][x] >= grid[y][x] then
      if dy < y then
        visible.top = false
      elseif dy > y then
        visible.bottom = false
      end
    end
  end
  return visible.top or visible.bottom or visible.left or visible.right;
end

local function get_scenic_score(x, y)
  local visible_count = {
    top = 0,
    bottom = 0,
    left = 0,
    right = 0,
  }
  if x == 1 or y == 1 or x == #grid[1] or y == #grid then
    return 0
  end
  local point = grid[y][x]
  for leftx = x - 1, 1, -1 do
    if grid[y][leftx] >= point then
      visible_count.left = visible_count.left + 1
      break
    end
    visible_count.left = visible_count.left + 1
  end
  for rightx = x + 1, #grid[1] do
    if grid[y][rightx] >= point then
      visible_count.right = visible_count.right + 1
      break
    end
    visible_count.right = visible_count.right + 1
  end
  for topy = y - 1, 1, -1 do
    if grid[topy][x] >= point then
      visible_count.top = visible_count.top + 1
      break
    end
    visible_count.top = visible_count.top + 1
  end
  for bottomy = y + 1, #grid do
    if grid[bottomy][x] >= point then
      visible_count.bottom = visible_count.bottom + 1
      break
    end
    visible_count.bottom = visible_count.bottom + 1
  end
  print('(' .. x .. ',' .. y .. ')', visible_count.top, visible_count.left, visible_count.bottom, visible_count.right)
  return visible_count.top * visible_count.bottom * visible_count.left * visible_count.right
end

local y = 0;
for line in io.lines() do
  y = y + 1;
  grid[y] = grid[y] or {}
  for x, char in line:gmatch("()(.)") do
    grid[y][x] = tonumber(char)
  end
end

local visible_count = 0
local max_scenic_score = 0
for y = 1, #grid do
  for x = 1, #grid[1] do
    local visible = is_visible(x, y)
    local scenic_score = get_scenic_score(x, y)
    if max_scenic_score < scenic_score then
      max_scenic_score = scenic_score
    end
    if visible then
      visible_count = visible_count + 1
    end
  end
end

print('Part 1:', visible_count)
print('Part 2:', max_scenic_score)
