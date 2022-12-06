use std::fs;

pub fn has_duplicate(buffer: &Vec<char>) -> bool {
    let mut i = 0;
    for c in buffer {
        let mut j = 0;
        for d in buffer {
            if i == j {
                j += 1;
                continue;
            }
            if *c == *d {
                return true;
            }
            j += 1;
        }
        i += 1;
    }
    return false;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input6.txt").expect("Couldn't read file.");
    let mut it = input.chars().into_iter();
    let signature_len = 14; // Change to 4 for puzzle 1, 14 for puzzle 2
    let mut buffer: Vec<char> = vec!['0'; signature_len];
    let mut index = 0;
    println!("{input}");
    loop {
        let c = it.next().unwrap_or('-');
        if c == '-' {
            return;
        }

        buffer[index % signature_len] = c;
        if !has_duplicate(&buffer) && index >= signature_len {
            let start = index + 1;
            println!("Start is {start}");
            break;
        }

        index += 1;
    }
}
