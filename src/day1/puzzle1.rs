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

    // Find the most caloric-dense elf
    calories = 0;
    for e in elves {
        if (e > calories) {
            calories = e;
        }
    }
    println!("Most calories is {calories}");
}
