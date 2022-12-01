use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input1-1.txt").expect("Couldn't read file.");
    let mut elves: Vec<i32> = Vec::new();
    let mut calories = 0;
    for token in input.split('\n') {
        if token == "" {
            elves.push(calories);
            println!("Created elf with rations: {calories}");
            calories = 0;
            continue;
        }
        calories += token.parse::<i32>().unwrap();
    }
    elves.push(calories);
    elves.sort();
    elves.reverse();
    const TOP_COUNT: usize = 3;
    let mut sum = 0;
    for e in &elves[..TOP_COUNT] {
        sum += e;
    }
    println!("Sum is {sum}");
}
