use std::fs;

fn dist(x1: i32, x2: i32) -> i32 {
    if x1 > x2 {
        x1 - x2
    } else {
        x2 - x1
    }
}

fn get_next_head(p: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "U" => (p.0, p.1 - 1),
        "D" => (p.0, p.1 + 1),
        "L" => (p.0 - 1, p.1),
        "R" => (p.0 + 1, p.1),
        _ => (0, 0),
    }
}

fn get_next_tail(head: (i32, i32), tail: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "U" => {
            if dist(head.1, tail.1) > 1 {
                (head.0, head.1 + 1)
            } else {
                tail
            }
        }
        "D" => {
            if dist(head.1, tail.1) > 1 {
                (head.0, head.1 - 1)
            } else {
                tail
            }
        }
        "L" => {
            if dist(head.0, tail.0) > 1 {
                (head.0 + 1, head.1)
            } else {
                tail
            }
        }
        "R" => {
            if dist(head.0, tail.0) > 1 {
                (head.0 - 1, head.1)
            } else {
                tail
            }
        }
        "LU" => {
            let dx = dist(head.0, tail.0);
            let dy = dist(head.1, tail.1);
            if dx > 1 && dy > 1 {
                (head.0 + 1, head.1 + 1)
            } else if dx > 1 {
                (head.0 + 1, head.1)
            } else if dy > 1 {
                (head.0, head.1 + 1)
            } else {
                tail
            }
        }
        "LD" => {
            let dx = dist(head.0, tail.0);
            let dy = dist(head.1, tail.1);
            if dx > 1 && dy > 1 {
                (head.0 + 1, head.1 - 1)
            } else if dx > 1 {
                (head.0 + 1, head.1)
            } else if dy > 1 {
                (head.0, head.1 - 1)
            } else {
                tail
            }
        }
        "RU" => {
            let dx = dist(head.0, tail.0);
            let dy = dist(head.1, tail.1);
            if dx > 1 && dy > 1 {
                (head.0 - 1, head.1 + 1)
            } else if dx > 1 {
                (head.0 - 1, head.1)
            } else if dy > 1 {
                (head.0, head.1 + 1)
            } else {
                tail
            }
        }
        "RD" => {
            let dx = dist(head.0, tail.0);
            let dy = dist(head.1, tail.1);

            if dx > 1 && dy > 1 {
                (head.0 - 1, head.1 - 1)
            } else if dx > 1 {
                (head.0 - 1, head.1)
            } else if dy > 1 {
                (head.0, head.1 - 1)
            } else {
                tail
            }
        }
        _ => tail,
    }
}

fn get_direction_moved(start: (i32, i32), end: (i32, i32)) -> String {
    let mut direction = String::from("");
    if end.0 > start.0 {
        direction.push('R');
    } else if end.0 < start.0 {
        direction.push('L');
    }
    if end.1 > start.1 {
        direction.push('D');
    } else if end.1 < start.1 {
        direction.push('U');
    }
    let sx = start.0;
    let sy = start.1;
    let ex = end.0;
    let ey = end.1;
    println!("Direction of ({sx}, {sy}) -> ({ex}, {ey}) is {direction}");
    return direction;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input9.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 500]; 500];
    let mut segments: Vec<(i32, i32)> = vec![(250, 250); 10];
    for line in lines {
        let mut it = line.split(' ').into_iter();
        let direction = it.next().unwrap();
        let steps = it.next().unwrap().parse::<i32>().unwrap();
        println!("{steps}");

        for _ in 0..steps {
            let head = get_next_head(segments[0], direction);
            let mut dir = direction.to_owned();
            segments[0] = head;
            println!("Head Moved direction: {dir}");
            for s in 1..segments.len() {
                let head = segments[s - 1];
                let tail = get_next_tail(head, segments[s], &dir);
                dir = get_direction_moved(segments[s], tail);
                if s == 1 {
                    println!("Tail Moved direction: {dir}");
                }
                segments[s] = tail;
            }
            let tail = segments[segments.len() - 1];
            let x = tail.0;
            let y = tail.1;
            grid[tail.1 as usize][tail.0 as usize] = true;
            /*
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    let mut occupied = false;
                    for s in 0..segments.len() {
                        if segments[s].0 as usize == x && segments[s].1 as usize == y {
                            print!("{s} ");
                            occupied = true;
                            break;
                        }
                    }
                    if !occupied {
                        print!(". ");
                    }
                }
                println!("");
            }*/
        }
    }
    let mut count = 0;
    for y in grid {
        for x in y {
            if x {
                count += 1;
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!("");
    }
    println!("Count 2: {count}");
}
