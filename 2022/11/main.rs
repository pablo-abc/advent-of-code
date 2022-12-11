use std::collections::VecDeque;
use std::io::stdin;

struct Monkey {
    pub items: VecDeque<i64>,
    pub operation: Box<dyn Fn(i64) -> i64>,
    pub test: Box<dyn Fn(i64) -> bool>,
    pub if_true: usize,
    pub if_false: usize,
    pub inspection_count: i64,
    pub divisor: i64,
}

impl Monkey {
    pub fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            operation: Box::new(|x| x),
            test: Box::new(|_| false),
            divisor: 1,
            if_true: 0,
            if_false: 0,
            inspection_count: 0,
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let use_lcm = if let Some(arg) = args.get(1) {
        arg == "--lcm"
    } else {
        false
    };
    let mut buffer = String::new();
    let mut eof = false;
    let mut monkeys: Vec<Monkey> = Vec::new();

    while !eof {
        buffer.clear();
        match stdin().read_line(&mut buffer) {
            Ok(0) => eof = true,
            Ok(_) => {
                let line = buffer.trim();
                let splitted = line.split(' ').collect::<Vec<&str>>();
                match splitted[0] {
                    "Monkey" => {
                        monkeys.push(Monkey::new());
                    }
                    "Starting" => {
                        let starting_line = line
                            .split(':')
                            .map(|l| l.trim().replace(",", ""))
                            .collect::<Vec<_>>();
                        monkeys.last_mut().unwrap().items = starting_line[1]
                            .split(' ')
                            .map(|i| i.parse::<i64>().unwrap())
                            .collect::<VecDeque<_>>();
                    }
                    "Operation:" => {
                        let operation_line = line.split(':').map(|l| l.trim()).collect::<Vec<_>>();
                        let fn_desc = operation_line[1]
                            .split(' ')
                            .skip(3)
                            .map(String::from)
                            .collect::<Vec<_>>();
                        monkeys.last_mut().unwrap().operation = Box::new(move |old: i64| {
                            let value = if fn_desc[1] == "old" {
                                old
                            } else {
                                fn_desc[1].parse::<i64>().unwrap()
                            };
                            match fn_desc[0].as_str() {
                                "+" => old + value,
                                "*" => old * value,
                                _ => panic!("Unknown operation"),
                            }
                        });
                    }
                    "Test:" => {
                        let test_line = line.split(" by ").map(|l| l.trim()).collect::<Vec<_>>();
                        let value = test_line[1].parse::<i64>().unwrap();
                        monkeys.last_mut().unwrap().divisor = value;
                        monkeys.last_mut().unwrap().test = Box::new(move |old| (old % value) == 0);
                    }
                    "If" => {
                        let if_line = line.split(" monkey ").map(|l| l.trim()).collect::<Vec<_>>();
                        let index = if_line[1].parse::<usize>().unwrap();
                        let monkey = monkeys.last_mut().unwrap();
                        match splitted[1] {
                            "true:" => {
                                monkey.if_true = index;
                            }
                            "false:" => {
                                monkey.if_false = index;
                            }
                            _ => panic!("Unknown if"),
                        };
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("Error reading stdin: {}", e),
        }
    }
    let lcm = {
        let divisors = &monkeys.iter().map(|m| m.divisor).collect::<Vec<_>>();
        divisors.iter().fold(1, |acc, x| acc * x)
    };
    let limit = if use_lcm { 10000 } else { 20 };
    for _ in 0..limit {
        for i in 0..monkeys.len() {
            {
                let monkey = &mut monkeys[i];
                let items_length = monkey.items.len();
                monkey.inspection_count += items_length as i64;
            }
            while !&monkeys[i].items.is_empty() {
                let if_true = monkeys[i].if_true;
                let if_false = monkeys[i].if_false;
                let worry_level =
                    get_worry_level(&mut monkeys[i], if use_lcm { Some(lcm) } else { None });
                let test = &monkeys[i].test;
                if test(worry_level) {
                    let true_monkey = &mut monkeys[if_true];
                    true_monkey.items.push_back(worry_level);
                } else {
                    let false_monkey = &mut monkeys[if_false];
                    false_monkey.items.push_back(worry_level);
                }
            }
        }
    }
    let mut frequency = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<_>>();
    frequency.sort();
    frequency.reverse();
    println!(
        "Part {}: {}",
        if use_lcm { "2" } else { "1" },
        frequency[0] * frequency[1]
    );
}

fn get_worry_level(monkey: &mut Monkey, lcm: Option<i64>) -> i64 {
    let operation = &monkey.operation;
    let items = &mut monkey.items;
    let item = items.pop_front().unwrap();

    if let Some(lcm) = lcm {
        operation(item) % lcm
    } else {
        operation(item) / 3
    }
}
