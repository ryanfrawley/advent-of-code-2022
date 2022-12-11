use std::{collections::VecDeque, fs, ops::IndexMut};

struct Monkey {
    items: VecDeque<u64>,
    operation: fn(old: u64, symbol: &str, operand: &str) -> u64,
    operand_symbol: String,
    operand: String,
    divisor: u64,
    throw_true: usize,
    throw_false: usize,
    inspected_items: u64,
    test: fn(worry: u64, divisor: u64) -> bool,
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    let mut i = 0;
    for monkey in monkeys {
        println!("Monkey {i}");
        print!("  ");
        for item in &monkey.items {
            print!("{item}, ");
        }
        println!("");
        i += 1;
    }
}

pub fn run() {
    let input = fs::read_to_string("inputs/input11.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut it = lines.into_iter();
    let mut index = 0;
    let mut global_divisor = 1;
    loop {
        let has_next = it.next();
        if has_next == None {
            break;
        }
        if index > 0 {
            it.next();
        }
        let mut tokens = it.next().unwrap().split_whitespace();

        // Starting items
        tokens.next();
        tokens.next();

        let mut items: VecDeque<u64> = VecDeque::new();
        for token in tokens {
            let numeric_token = token.replace(",", "");
            items.push_back(numeric_token.parse::<u64>().unwrap());
            println!("Added token {numeric_token}");
        }

        // Operation
        //   Operation: new = old * 19
        tokens = it.next().unwrap().split_whitespace();
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        let symbol = tokens.next().unwrap();
        let operand = tokens.next().unwrap();
        println!("Op: new = old {symbol} {operand}");

        // Test
        // Test: divisible by 23
        tokens = it.next().unwrap().split_whitespace();
        tokens.next();
        tokens.next();
        tokens.next();
        let divisible_by = tokens.next().unwrap().parse::<u64>().unwrap();
        global_divisor *= divisible_by;

        println!("Divisble by {divisible_by}");

        // Test branch true
        tokens = it.next().unwrap().split_whitespace();
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        let throw_true = tokens.next().unwrap().parse::<usize>().unwrap();

        // Test branch false
        tokens = it.next().unwrap().split_whitespace();
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        let throw_false = tokens.next().unwrap().parse::<usize>().unwrap();

        monkeys.push(Monkey {
            inspected_items: 0,
            items,
            operand: operand.to_owned(),
            operand_symbol: symbol.to_owned(),
            throw_true,
            throw_false,
            divisor: divisible_by,
            operation: |old, symbol, operand| {
                let operand = match operand {
                    "old" => old,
                    _ => operand.parse::<u64>().unwrap(),
                };
                let result = match symbol {
                    "*" => old * operand,
                    "+" => old + operand,
                    _ => 0,
                };
                result
            },
            test: |worry, divisor| worry % divisor == 0,
        });

        index += 1;
    }
    print_monkeys(&monkeys);
    for _round in 0..10000 {
        println!("---");
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.index_mut(i);
            let mut items = &mut monkey.items;
            let mut moved_items: Vec<(usize, u64)> = Vec::new();
            loop {
                let item = items.pop_front();
                match item {
                    None => break,
                    Some(_) => {
                        let worry = (monkey.operation)(
                            item.unwrap(),
                            &monkey.operand_symbol,
                            &monkey.operand,
                        ) % global_divisor;
                        let catch_index = match (monkey.test)(worry, monkey.divisor) {
                            true => monkey.throw_true,
                            false => monkey.throw_false,
                        } as usize;
                        moved_items.push((catch_index, worry));
                        monkey.inspected_items += 1;
                    }
                }
            }
            for moved_item in moved_items {
                let monkey = monkeys.index_mut(moved_item.0);
                monkey.items.push_back(moved_item.1);
            }
        }
        print_monkeys(&monkeys);
    }
    let mut top1 = 0;
    let mut top2 = 0;
    for i in 0..monkeys.len() {
        let inspected_items = monkeys.get(i).unwrap().inspected_items;
        if inspected_items > top1 {
            top2 = top1;
            top1 = inspected_items;
        } else if inspected_items > top2 {
            top2 = inspected_items;
        }
        println!("Monkey {i}: {inspected_items}");
    }
    let monkey_business = top1 * top2;
    println!("Monkey business: {monkey_business}");
}
