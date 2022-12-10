use std::{collections::HashSet, io::stdin, str::FromStr};

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl ToString for Position {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl FromStr for Movement {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Movement::Up),
            "D" => Ok(Movement::Down),
            "L" => Ok(Movement::Left),
            "R" => Ok(Movement::Right),
            _ => Err("Failed to parse movement"),
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let rope_length = args[1].parse::<usize>().unwrap();
    let mut buffer = String::new();
    let mut eol = false;
    let mut rope = Vec::new();
    rope.resize(rope_length, Position { x: 0, y: 0 });
    let mut visited = HashSet::new();

    while !eol {
        buffer.clear();
        match stdin().read_line(&mut buffer) {
            Ok(0) => eol = true,
            Ok(_) => {
                let mut splitted = buffer
                    .trim()
                    .split(' ')
                    .map(String::from)
                    .collect::<Vec<String>>();
                let amount = splitted.pop().unwrap().parse::<usize>().unwrap();
                let movement = splitted.pop().unwrap().parse::<Movement>().unwrap();
                for _ in 0..amount {
                    match movement {
                        Movement::Up => {
                            rope[0].y += 1;
                        }
                        Movement::Down => {
                            rope[0].y -= 1;
                        }
                        Movement::Left => {
                            rope[0].x -= 1;
                        }
                        Movement::Right => {
                            rope[0].x += 1;
                        }
                    }
                    for i in 1..rope.len() {
                        let mut head_position = rope[i - 1].clone();
                        let mut tail_position = rope[i].clone();
                        follow_head(&mut head_position, &mut tail_position);
                        rope[i] = tail_position;
                    }
                    visited.insert(rope.last().unwrap().to_string());
                }
            }
            Err(e) => panic!("Error reading stdin: {}", e),
        };
    }
    println!(
        "Unique positions for tail of rope of length {}: {}",
        rope_length,
        visited.len()
    );
}

fn follow_head(head_position: &mut Position, tail_position: &mut Position) {
    if (head_position.y - tail_position.y) > 1 && (head_position.x - tail_position.x) > 1 {
        tail_position.y += 1;
        tail_position.x += 1;
    } else if (head_position.y - tail_position.y) > 1 && (head_position.x - tail_position.x) < -1 {
        tail_position.y += 1;
        tail_position.x -= 1;
    } else if (head_position.y - tail_position.y) < -1 && (head_position.x - tail_position.x) > 1 {
        tail_position.y -= 1;
        tail_position.x += 1;
    } else if (head_position.y - tail_position.y) < -1 && (head_position.x - tail_position.x) < -1 {
        tail_position.y -= 1;
        tail_position.x -= 1;
    } else if head_position.y - tail_position.y > 1 {
        tail_position.y += 1;
        tail_position.x += head_position.x - tail_position.x;
    } else if (head_position.y - tail_position.y) < -1 {
        tail_position.y -= 1;
        tail_position.x += head_position.x - tail_position.x;
    } else if (head_position.x - tail_position.x) > 1 {
        tail_position.x += 1;
        tail_position.y += head_position.y - tail_position.y;
    } else if (head_position.x - tail_position.x) < -1 {
        tail_position.x -= 1;
        tail_position.y += head_position.y - tail_position.y;
    }
}
