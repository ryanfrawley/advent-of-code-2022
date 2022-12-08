use std::fs;

// TODO: Read up on Box, Pin, and unsafe mode. I cheated and used JS for this puzzle.

struct Directory<'a> {
    path: &'a str,
    size: i32,
    subdirectories: Vec<Directory<'a>>,
}

fn calculate_answer(dir: &Directory) -> i32 {
    let mut sizes: Vec<i32> = Vec::new();
    calculate_dir_size(dir, &mut sizes);
    let mut answer = 0;
    for size in sizes {
        answer += size;
    }
    return answer;
}

fn calculate_dir_size(dir: &Directory, sizes: &mut Vec<i32>) -> i32 {
    let mut size = dir.size;
    for sub in &dir.subdirectories {
        let sub_size = calculate_dir_size(sub, sizes);
        if sub_size <= 100000 {
            sizes.push(sub_size);
        }
        size += sub_size;
    }
    return size;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input7-test.txt").expect("Couldn't read file.");
    let mut lines = input.split('\n');
    let mut command = "";
    let mut root: Directory = Directory {
        path: "/",
        size: 0,
        subdirectories: Vec::new(),
    };
    let mut dir = &mut root;
    for line in lines {
        let mut tokens = line.split(' ');
        let mut token = tokens.next();

        // Check for command pattern
        if token == Some("$") {
            command = tokens.next().unwrap();
            println!("Set command {command}");
        } else {
            // Continue from previous command
            if command == "ls" {
                let tokenUnwrapped = token.unwrap();
                if tokenUnwrapped.is_ascii() {
                    // Directory
                    token = tokens.next();
                    dir.subdirectories.push(Directory {
                        path: token.unwrap(),
                        subdirectories: Vec::new(),
                        size: 0,
                    });
                } else {
                    // File
                    let size = tokenUnwrapped.parse::<i32>().unwrap();
                    dir.size += size;
                }
            }
        }
        if command == "cd" {
            let next_path = tokens.next().unwrap();
            for d in dir.subdirectories.iter() {
                if d.path == next_path {
                    println!("updated path to {next_path}");
                    dir = &mut d;
                }
            }
        }
    }
    // Climb back to the top
    let answer = calculate_answer(&dir);
    println!("The answer is: {answer}");
}
