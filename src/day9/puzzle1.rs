use std::fs;

fn dist(x1: i32, x2: i32) -> i32 {
    if x1 > x2 {
        x1 - x2
    } else {
        x2 - x1
    }
}

pub fn run() {
    let input = fs::read_to_string("inputs/input9.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    let mut head = (500, 500);
    let mut tail = (500, 500);
    for line in lines {
        let mut it = line.split(' ').into_iter();
        let direction = it.next().unwrap();
        let steps = it.next().unwrap().parse::<i32>().unwrap();
        println!("{steps}");
        for _ in 0..steps {
            match direction {
                "U" => {
                    head.1 -= 1;
                    if dist(head.1, tail.1) > 1 {
                        tail.0 = head.0;
                        tail.1 = head.1 + 1;
                    }
                }
                "D" => {
                    head.1 += 1;
                    if dist(head.1, tail.1) > 1 {
                        tail.0 = head.0;
                        tail.1 = head.1 - 1;
                    }
                }
                "L" => {
                    head.0 -= 1;
                    if dist(head.0, tail.0) > 1 {
                        tail.1 = head.1;
                        tail.0 = head.0 + 1;
                    }
                }
                "R" => {
                    head.0 += 1;
                    if dist(head.0, tail.0) > 1 {
                        tail.1 = head.1;
                        tail.0 = head.0 - 1;
                    }
                }
                _ => {}
            }
            let tx = tail.0;
            let ty = tail.1;
            println!("tail: {tx}, {ty}");
            grid[tail.1 as usize][tail.0 as usize] = true;
        }
    }
    let mut count = 0;
    for y in grid {
        for x in y {
            if x {
                count += 1;
            }
        }
    }
    println!("Count: {count}");
}
