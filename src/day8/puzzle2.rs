use std::fs;

fn is_inside_treeline(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x > 0 && x < width - 1 && y > 0 && y < height - 1
}

fn scenic_score(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    if !is_inside_treeline(x, y, grid) {
        return 0;
    }
    let tree_height = grid[y][x];

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    // Up
    let mut temp_y = y;
    while temp_y > 0 {
        temp_y -= 1;
        up += 1;
        if grid[temp_y][x] >= tree_height {
            break;
        }
    }

    // Down
    temp_y = y;
    while temp_y < grid.len() - 1 {
        temp_y += 1;
        down += 1;
        if grid[temp_y][x] >= tree_height {
            break;
        }
    }

    // Left
    let mut temp_x = x;
    while temp_x > 0 {
        temp_x -= 1;
        left += 1;
        if grid[y][temp_x] >= tree_height {
            break;
        }
    }

    // Right
    temp_x = x;
    while temp_x < grid[0].len() - 1 {
        temp_x += 1;
        right += 1;
        if grid[y][temp_x] >= tree_height {
            break;
        }
    }

    println!("{up} - {down} - {left} - {right}");

    up * down * left * right
}

fn find_best_scenery(grid: &Vec<Vec<u32>>) -> u32 {
    let mut best = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let score = scenic_score(x, y, grid);
            if (score > best) {
                best = score;
            }
        }
    }
    return best;
}

pub fn run() {
    let input = fs::read_to_string("inputs/input8.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let chars = line.chars();
        let mut row: Vec<u32> = Vec::new();
        for char in chars {
            let num = char.to_digit(10);
            row.push(num.unwrap());
        }
        grid.push(row);
    }
    let x = grid[0].len();
    let y = grid.len();
    println!("Size: {x}x{y}");

    let count = find_best_scenery(&grid);
    println!("Best score: {count}");
}
