use std::io::stdin;

fn get_priority(char: char) -> u32 {
    if char.is_lowercase() {
        char as u32 - 96
    } else {
        char as u32 - 38
    }
}

fn main() {
    let mut buffer = String::new();
    let mut eof = false;
    let mut rucksack_sum = 0;
    while !eof {
        buffer.clear();
        match stdin().read_line(&mut buffer) {
            Ok(0) => eof = true,
            Ok(_) => {
                // Part 1
                let line = buffer.trim().to_string();
                let split_pos = line.len() / 2;
                let mut first_compartment = line[..split_pos].chars().collect::<Vec<_>>();
                first_compartment.sort();
                first_compartment.dedup();
                let mut second_compartment = line[split_pos..].chars().collect::<Vec<_>>();
                second_compartment.sort();
                second_compartment.dedup();
                rucksack_sum += first_compartment
                    .into_iter()
                    .filter(|c| second_compartment.contains(c))
                    .map(|c| get_priority(c))
                    .sum::<u32>();
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    println!("Part 1: {}", rucksack_sum);
}
