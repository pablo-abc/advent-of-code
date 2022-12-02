local points = {
  X = 1,
  Y = 2,
  Z = 3,
  A = 1,
  B = 2,
  C = 3,
}

local function get_losing(oponent)
  return ({ 3, 1, 2 })[oponent]
end

local function get_winning(oponent)
  return ({ 2, 3, 1 })[oponent]
end

local function identity(v) return v end

local choice = {
  X = get_losing,
  Y = identity,
  Z = get_winning,
}

local function get_score(oponent, mine)
  if (oponent == mine) then return 3 + mine end
  local winning_position = get_winning(oponent)
  if (winning_position == mine) then
    return 6 + mine
  else
    return mine
  end
end

local total_score = { 0, 0 }

for line in io.lines() do
  local oponent = points[line:sub(1, 1)]
  local outcome = line:sub(3, 3)
  local mine = points[outcome]
  total_score[1] = total_score[1] + get_score(oponent, mine)
  total_score[2] = total_score[2] + get_score(oponent, choice[outcome](oponent))
end

print('Part 1:', total_score[1])
print('Part 2:', total_score[2])
