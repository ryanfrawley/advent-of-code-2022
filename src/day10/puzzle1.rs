use std::fs;

fn signal_strength(cycle: i32, x: i32) -> i32 {
    cycle * x
}

pub fn run() {
    let input = fs::read_to_string("inputs/input10.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut cycle = 1;
    let mut signal = 0;
    let mut x = 1;
    for line in lines {
        let mut tokens = line.split(' ');
        let increment: i32;
        let opcode = tokens.next();
        match opcode {
            Some("noop") => {
                increment = 1;
            }
            Some("addx") => {
                increment = 2;
            }
            None => {
                break;
            }
            _ => {
                break;
            }
        }

        // Run the clock
        for i in 0..increment {
            if i == increment - 1 {
                // Opcode has finished
                match opcode {
                    Some("addx") => {
                        x += tokens.next().unwrap().parse::<i32>().unwrap();
                    }
                    _ => {}
                }
            }
            cycle += 1;
            if cycle == 20 || (cycle + 20) % 40 == 0 {
                signal += signal_strength(cycle, x);
                println!("Cycle: {cycle} - x: {x} - signal: {signal}");
            }
        }
    }
    println!("Signal: {signal}");
}
