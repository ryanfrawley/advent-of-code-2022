use std::fs;

fn is_inside_treeline(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x > 0 && x < width - 1 && y > 0 && y < height - 1
}

fn is_visible(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
    if !is_inside_treeline(x, y, grid) {
        return true;
    }
    let tree_height = grid[y][x];

    println!("tree {x}, {y} height: {tree_height}");
    let mut up = true;
    let mut down = true;
    let mut left = true;
    let mut right = true;

    // Up
    let mut temp_y = y;
    while temp_y > 0 {
        temp_y -= 1;
        if grid[temp_y][x] >= tree_height {
            let tmp = grid[temp_y][x];
            up = false;
            break;
        }
    }

    // Down
    temp_y = y;
    while temp_y < grid.len() - 1 {
        temp_y += 1;
        if grid[temp_y][x] >= tree_height {
            let tmp = grid[temp_y][x];
            down = false;
            break;
        }
    }

    // Left
    let mut temp_x = x;
    while temp_x > 0 {
        temp_x -= 1;
        if grid[y][temp_x] >= tree_height {
            left = false;
            break;
        }
    }

    // Right
    temp_x = x;
    while temp_x < grid[0].len() - 1 {
        temp_x += 1;
        if grid[y][temp_x] >= tree_height {
            right = false;
            break;
        }
    }

    up || down || left || right
}

fn calc_visible_trees(grid: &Vec<Vec<u32>>) -> u32 {
    let mut count: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_visible(x, y, grid) {
                count += 1;
            }
        }
    }
    return count;
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

    let count = calc_visible_trees(&grid);
    println!("Visible trees: {count}");
}
