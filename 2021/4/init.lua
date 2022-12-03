local function strsplit(str, sep)
  local values = {}
  for value in str:gmatch('[^' .. sep .. ']+') do
    table.insert(values, value)
  end
  return values
end

local numbers = {}
local boards = {}

for line in io.lines() do
  if #numbers == 0 then
    numbers = strsplit(line, ',')
  elseif (line == '') then
    table.insert(boards, {})
  else
    table.insert(boards[#boards], strsplit(line, ' '))
  end
end

local function mark(board, x, y)
  board[y][x] = 'x'
end

local function check_horizontal_win(board)
  for y = 1, #board do
    for x = 1, #board do
      if 'x' ~= board[y][x] then break
      elseif x == #board[1] then return true end
    end
  end
  return false
end

local function check_vertical_win(board)
  for x = 1, #board do
    for y = 1, #board do
      if 'x' ~= board[y][x] then break
      elseif y == #board[1] then return true end
    end
  end
  return false
end

local function sum_unmarked(board)
  local sum = 0
  for x = 1, #board do
    for y = 1, #board do
      local value = board[y][x]
      if (value ~= 'x') then
        sum = sum + value
      end
    end
  end
  return sum
end

local function play()
  local first_score = nil
  local won_count = 0
  local won_boards = {}
  for _, number in ipairs(numbers) do
    for index, board in ipairs(boards) do
      for y = 1, #board do
        for x = 1, #board[y] do
          if board[y][x] == number then
            mark(board, x, y)
            local won = check_horizontal_win(board) or check_vertical_win(board)
            if (won and not won_boards[index]) then
              won_boards[index] = true
              won_count = won_count + 1
              if (not first_score) then
                first_score = sum_unmarked(board) * number
              end
              if (won_count == #boards) then
                return first_score, sum_unmarked(board) * number
              end
              break
            end
          end
        end
      end
    end
  end
end

local first_score, last_score = play()

print('Part 1:', first_score)
print('Part 2:', last_score)
