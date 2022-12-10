use std::fs;

fn update_pixel(cycle: i32, rx: i32, display: &mut Vec<Vec<bool>>) {
    let y = (cycle - 1) / 40;
    let x = (cycle - 1) % 40;
    if x <= rx + 1 && x >= rx - 1 {
        display[y as usize][x as usize] = true;
    }
}

pub fn run() {
    let input = fs::read_to_string("inputs/input10.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut cycle = 1;
    let mut x = 1;
    let mut display = vec![vec![false; 40]; 6];
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
            update_pixel(cycle, x, &mut display);
            cycle += 1;
            if i == increment - 1 {
                // Opcode has finished
                match opcode {
                    Some("addx") => {
                        x += tokens.next().unwrap().parse::<i32>().unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
    for row in display {
        for pixel in row {
            match pixel {
                true => print!("# "),
                false => print!(". "),
            }
        }
        println!("");
    }
}
