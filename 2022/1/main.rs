use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    let mut eof = false;
    let mut elfs: Vec<Vec<u32>> = Vec::new();
    while !eof {
        match stdin().read_line(&mut buffer) {
            Ok(0) => eof = true,
            Ok(_) => {
                let line = buffer.trim().to_string();
                buffer.clear();
                if elfs.len() == 0 || line == "" {
                    elfs.push(Vec::new());
                }
                if line != "" {
                    elfs.last_mut()
                        .expect("Error obtaining last_mut")
                        .push(line.parse().expect("Line is not a number"));
                }
            }
            Err(e) => panic!("Error reading stdin: {}", e),
        }
    }

    let mut sums = elfs
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", sums[0]);
    println!("Part 2: {}", sums.into_iter().take(3).sum::<u32>());
}
