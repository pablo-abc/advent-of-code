local root = {
  name = '/',
  files = {},
  directories = {},
  parent = nil
}

local current = root;

local function find_directory(name)
  for _, directory in ipairs(current.directories) do
    if directory.name == name then
      return directory
    end
  end
end

for line in io.lines() do
  local command, arg = line:match('^%$ ([a-z]+) (.+)$')
  local dir_name = line:match('^dir (.+)$')
  local size, file_name = line:match('^([0-9]+) (.+)$')
  if command == 'cd' then
    if arg == '..' then
      current = current.parent
    elseif arg == '/' then
      current = root
    else
      current = find_directory(arg) or { name = arg, files = {}, directories = {}, parent = current }
    end
  elseif dir_name then
    local directory = find_directory(dir_name) or { name = dir_name, files = {}, directories = {}, parent = current }
    table.insert(current.directories, directory)
  elseif file_name then
    table.insert(current.files, { name = file_name, size = tonumber(size) })
  end
end

local directories = {}

local function find_size(root_dir)
  local size = 0
  for _, file in ipairs(root_dir.files) do
    size = size + file.size
  end
  for _, directory in ipairs(root_dir.directories) do
    local dir_size = find_size(directory)
    size = size + dir_size
    table.insert(directories, dir_size)
  end
  return size
end

table.insert(directories, find_size(root))

table.sort(directories, function(a, b) return a > b end)

local sum_under_100000 = 0
local unused_space = 70000000 - directories[1]
local needed_space = 30000000 - unused_space
local last_size = directories[1]
local smallest_needed_size = 0

for _, size in ipairs(directories) do
  if size <= 100000 then
    sum_under_100000 = sum_under_100000 + size
  end
  if size < needed_space and last_size >= needed_space then
    smallest_needed_size = last_size
  end
  last_size = size
end

print('Part 1:', sum_under_100000)
print('Part 2:', smallest_needed_size)
