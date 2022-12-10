use std::io::stdin;
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(' ').collect::<Vec<&str>>();
        let instruction = splitted[0];
        match instruction {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let value = splitted[1].parse::<i32>().unwrap();
                Ok(Instruction::AddX(value))
            }
            _ => Err("Failed to parse instruction"),
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let mut eof = false;
    let mut cycles: i32 = 0;
    let mut register_x = 1;
    let mut interesting_values = Vec::new();
    let mut screen = vec![repeat(".").take(40).collect::<String>()];

    while !eof {
        buffer.clear();
        match stdin().read_line(&mut buffer) {
            Ok(0) => eof = true,
            Ok(_) => {
                let line = buffer.trim();
                let instruction = line.parse::<Instruction>().unwrap();
                match instruction {
                    Instruction::AddX(x) => {
                        cycles += 1;
                        update_interesting_values(&mut interesting_values, &mut cycles, register_x);
                        draw(&mut screen, cycles, register_x);
                        cycles += 1;
                        update_interesting_values(&mut interesting_values, &mut cycles, register_x);
                        draw(&mut screen, cycles, register_x);
                        register_x += x;
                    }
                    Instruction::Noop => {
                        cycles += 1;
                        update_interesting_values(&mut interesting_values, &mut cycles, register_x);
                        draw(&mut screen, cycles, register_x);
                    }
                }
            }
            Err(e) => panic!("Error reading stdin: {}", e),
        }
    }

    println!("Part 1: {}", interesting_values.into_iter().sum::<i32>());
    println!("Part 2:\n{}", screen.join("\n"));
}

fn update_interesting_values(interesting_values: &mut Vec<i32>, cycles: &mut i32, register_x: i32) {
    if *cycles == 20 || (*cycles - 20) % 40 == 0 {
        interesting_values.push(register_x * *cycles);
    }
}

fn draw(screen: &mut Vec<String>, cycles: i32, register_x: i32) {
    let start = register_x - 1;
    let end = register_x + 1;
    let current_crt = ((cycles as usize - 1) % 40) + 1;
    if cycles % 40 == 0 {
        screen.push(repeat(".").take(40).collect::<String>());
    }
    let line = screen.last_mut().unwrap();
    if start <= (current_crt - 1) as i32 && end >= (current_crt - 1) as i32 {
        line.replace_range(current_crt - 1..current_crt, "#");
    }
}
