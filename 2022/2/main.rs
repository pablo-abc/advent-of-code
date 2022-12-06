use std::collections::HashMap;
use std::io::stdin;

fn get_losing(oponent: u32) -> u32 {
    match oponent {
        1 => 3,
        2 => 1,
        3 => 2,
        _ => unreachable!(),
    }
}

fn get_winning(oponent: u32) -> u32 {
    match oponent {
        1 => 2,
        2 => 3,
        3 => 1,
        _ => unreachable!(),
    }
}

fn get_score(oponent: u32, mine: u32) -> u32 {
    if oponent == mine {
        3 + mine
    } else if get_winning(oponent) == mine {
        6 + mine
    } else {
        mine
    }
}

fn main() {
    let mut points = HashMap::new();
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);
    points.insert("A", 1);
    points.insert("B", 2);
    points.insert("C", 3);
    let mut buffer = String::new();
    let mut eof = false;
    let mut total_score = vec![0, 0];

    while !eof {
        buffer.clear();
        match stdin().read_line(&mut buffer) {
            Ok(0) => eof = true,
            Ok(_) => {
                let line = buffer.trim().to_string();
                let splitted = line.split(' ').collect::<Vec<&str>>();
                let oponent = points[splitted[0]];
                let outcome = splitted[1];
                let mine = points[outcome];
                total_score[0] = total_score[0] + get_score(oponent, mine);
                total_score[1] = total_score[1]
                    + get_score(
                        oponent,
                        match outcome {
                            "X" => get_losing(oponent),
                            "Y" => oponent,
                            "Z" => get_winning(oponent),
                            _ => unreachable!(),
                        },
                    )
            }
            Err(e) => panic!("Error reading input: {}", e),
        };
    }

    println!("Part 1: {}", total_score[0]);
    println!("Part 2: {}", total_score[1]);
}
