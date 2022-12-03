use std::fs;

fn get_priority(c: char) -> u8 {
    if c >= 'A' && c <= 'Z' {
        return ((27 + c as i8) - 'A' as i8) as u8;
    }
    return ((1 + c as i8) - 'a' as i8) as u8;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input3-1.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut priority_sum: i32 = 0;
    for line in lines {
        let mut rucks = vec![0; 26 * 2 + 1];
        let compartment1 = &line[..line.len() / 2];
        let compartment2 = &line[line.len() / 2..];
        println!("c1: {compartment1}, c2: {compartment2}");
        for i in compartment1.chars() {
            rucks[get_priority(i) as usize] = 1;
        }
        for i in compartment2.chars() {
            let ruck_item = rucks[get_priority(i) as usize];
            if ruck_item == 1 {
                priority_sum += get_priority(i) as i32;
                rucks[get_priority(i) as usize] += 1;
                let priority = get_priority(i);
                println!("Set rucks[{priority}] to: {i}");
                break;
            }
        }
    }
    println!("Priority sum: {priority_sum}");
}
