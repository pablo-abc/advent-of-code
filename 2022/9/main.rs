use std::{io::stdin, str::FromStr};

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
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
    let mut buffer = String::new();
    let mut eol = false;
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut trajectory = Vec::new();

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
                            head_position.y += 1;
                        }
                        Movement::Down => {
                            head_position.y -= 1;
                        }
                        Movement::Left => {
                            head_position.x -= 1;
                        }
                        Movement::Right => {
                            head_position.x += 1;
                        }
                    }
                    follow_head(&mut head_position, &mut tail_position);
                    trajectory.push(tail_position.to_string());
                }
            }
            Err(e) => panic!("Error reading stdin: {}", e),
        };
    }
    let mut deduped = trajectory.clone();
    deduped.sort();
    deduped.dedup();
    println!("Part 1: {}", deduped.len());
}

fn follow_head(head_position: &mut Position, tail_position: &mut Position) {
    if head_position.y - tail_position.y > 1 {
        tail_position.y += 1;
        tail_position.x += head_position.x - tail_position.x;
    } else if (head_position.y - tail_position.y) < -1 {
        tail_position.y -= 1;
        tail_position.x += head_position.x - tail_position.x;
    }

    if (head_position.x - tail_position.x) > 1 {
        tail_position.x += 1;
        tail_position.y += head_position.y - tail_position.y;
    } else if (head_position.x - tail_position.x) < -1 {
        tail_position.x -= 1;
        tail_position.y += head_position.y - tail_position.y;
    }
}
