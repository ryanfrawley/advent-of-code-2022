use std::fs;

fn get_priority(c: char) -> u8 {
    if c >= 'A' && c <= 'Z' {
        return ((27 + c as i8) - 'A' as i8) as u8;
    }
    return ((1 + c as i8) - 'a' as i8) as u8;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input3-1.txt").expect("Couldn't read file.");
    let mut lines = input.split('\n').into_iter();
    let mut priority_sum: i32 = 0;
    loop {
        let mut elves: Vec<&str> = Vec::new();
        let next = lines.next().unwrap_or("-");
        if next == "-" {
            break;
        }
        elves.push(next);
        elves.push(lines.next().unwrap());
        elves.push(lines.next().unwrap());
        let mut flag = 1;
        let mut rucks = vec![0; 26 * 2 + 1];
        for e in elves {
            for i in e.chars() {
                rucks[get_priority(i) as usize] |= flag;
            }
            flag <<= 1;
        }

        // Find the common one, which would be 1 & 2 & 4
        for i in 0..(26 * 2 + 1) {
            if rucks[i] == 1 | 2 | 4 {
                let r = rucks[i];
                priority_sum += i as i32;
            }
        }
    }

    println!("Priority sum: {priority_sum}");
}
