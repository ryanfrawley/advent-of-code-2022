use std::fs;

fn parse_range(range: &str) -> (i32, i32) {
    let mut it = range.split('-').into_iter();
    let n1 = it.next().unwrap().parse::<i32>().unwrap();
    let n2 = it.next().unwrap().parse::<i32>().unwrap();
    return (n1, n2);
}

fn fully_contained(assignment1: &str, assignment2: &str) -> bool {
    let range1 = parse_range(assignment1);
    let range2 = parse_range(assignment2);
    return range1.0 >= range2.0 && range1.1 <= range2.1
        || range2.0 >= range1.0 && range2.1 <= range1.1;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input4.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut overlaps = 0;
    for l in lines {
        let pair = l.split(',');
        let mut it = pair.into_iter();
        let assignment1 = it.next().unwrap();
        let assignment2 = it.next().unwrap();
        if fully_contained(assignment1, assignment2) {
            overlaps += 1;
        }
    }
    println!("Overlaps: {overlaps}");
}
