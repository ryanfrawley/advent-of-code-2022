use std::{cmp, collections::VecDeque, fs, u32::MAX};

type Point = (i32, i32);
struct GridPoint {
    x: usize,
    y: usize,
    altitude: u8,
    c: char,
    neighbors: Vec<Point>,
    parent: Option<Point>,
    f: u32,
    g: u32,
    h: u32,
}
type Grid = Vec<Vec<GridPoint>>;

type Vector = (i32, i32);

const UP: Vector = (-1, 0);
const DOWN: Vector = (1, 0);
const LEFT: Vector = (0, -1);
const RIGHT: Vector = (0, 1);

const DIRECTIONS: [Vector; 4] = [UP, DOWN, LEFT, RIGHT];

fn can_move(grid: &Grid, from: Point, to: Point) -> bool {
    if to.0 as usize > grid.len() - 1 || to.0 < 0 || to.1 as usize > grid[0].len() - 1 || to.1 < 0 {
        return false;
    }

    grid[from.0 as usize][from.1 as usize].altitude + 1
        >= grid[to.0 as usize][to.1 as usize].altitude
}

fn add_vector(p: Point, v: Vector) -> Point {
    (p.0 as i32 + v.1, p.1 as i32 + v.0)
}

fn add_neighbors(grid: &mut Grid, p: Point) {
    let mut neighbors: Vec<Point> = Vec::new();
    for d in DIRECTIONS {
        let neighbor = add_vector(p, d);
        let mut can_move_res = false;
        if can_move(grid, p, neighbor) {
            neighbors.push(neighbor);
            can_move_res = true;
        }
        let x = neighbor.1;
        let y = neighbor.0;
        if p.1 == 7 && p.0 == 4 {
            println!("Neighbor ({x}, {y}) - {can_move_res}");
        }
    }
    grid[p.0 as usize][p.1 as usize].neighbors = neighbors;
}

fn print_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            let c = col.c;
            print!("{c}");
        }
        println!("");
    }
}

fn get_altitude(c: char) -> u8 {
    let c = match c {
        'S' => 'a',
        'E' => 'z',
        _ => c,
    };
    ((c as i8) - 'a' as i8) as u8
}

fn h(p1: Point, p2: Point) -> u32 {
    let dx = i32::abs_diff(p1.1, p2.1);
    let dy = i32::abs_diff(p1.0, p2.0);
    dx + dy
}

fn reset_grid(grid: &mut Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = &mut grid[y][x];
            c.f = 0;
            c.g = 0;
            c.h = 0;
            c.parent = None;
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("inputs/input12.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut grid: Grid = Vec::new();
    let mut start: Vec<Point> = Vec::new();
    let mut end: Point = (0, 0);
    let mut y: usize = 0;
    for line in lines {
        let mut grid_row: Vec<GridPoint> = Vec::new();

        let mut x: usize = 0;
        for c in line.chars() {
            grid_row.push(GridPoint {
                x,
                y,
                c,
                altitude: get_altitude(c),
                neighbors: Vec::new(),
                parent: None,
                f: 0,
                g: 0,
                h: 0,
            });
            match c {
                'S' | 'a' => start.push((y as i32, x as i32)),
                'E' => end = (y as i32, x as i32),
                _ => {}
            }
            x += 1;
        }
        grid.push(grid_row);
        y += 1;
    }
    print_grid(&grid);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            add_neighbors(&mut grid, (y as i32, x as i32));
        }
    }

    let mut shortest = -1;
    for z in 0..start.len() {
        // reset the grid
        reset_grid(&mut grid);

        let mut open: VecDeque<Point> = VecDeque::new();
        let mut closed: VecDeque<Point> = VecDeque::new();
        open.push_back(start[z]);

        while open.len() > 0 {
            let mut lowest_index = 0;
            for i in 0..open.len() {
                if grid[open[i].0 as usize][open[i].1 as usize].f
                    < grid[open[lowest_index].0 as usize][open[lowest_index].1 as usize].f
                {
                    lowest_index = i
                }
            }
            let current = open[lowest_index];
            if current.1 == end.1 && current.0 == end.0 {
                let mut temp = &grid[current.0 as usize][current.1 as usize];
                let mut steps = 0;
                let mut killed = false;
                loop {
                    match temp.parent {
                        None => break,
                        Some(p) => {
                            let (y, x) = p;
                            temp = &grid[p.0 as usize][p.1 as usize]
                        }
                    }
                    if steps > shortest && shortest > 0 {
                        killed = true;
                        break;
                    }
                    steps += 1;
                }
                if !killed && steps < shortest || shortest == -1 {
                    shortest = steps;
                    println!("Shortest path: {steps}");
                }
                break;
            }
            open.remove(lowest_index);
            closed.push_back(current);

            let neighbor_positions =
                grid[current.0 as usize][current.1 as usize].neighbors[..].to_vec();
            let pos: Vec<Point> = neighbor_positions.to_vec();
            for neighbor_pos in pos {
                let (y, x) = neighbor_pos;
                // println!("Neighbor: ({x}, {y})");
                let neighbor = &grid[neighbor_pos.0 as usize][neighbor_pos.1 as usize];
                let mut found = false;
                for j in 0..closed.len() {
                    if closed[j].0 == neighbor.y as i32 && closed[j].1 == neighbor.x as i32 {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let g = grid[current.0 as usize][current.1 as usize].g + 1;

                    let mut found = false;
                    for j in 0..open.len() {
                        if open[j].0 == neighbor_pos.0 as i32 && open[j].1 == neighbor_pos.1 as i32
                        {
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        open.push_back((neighbor_pos.0 as i32, neighbor_pos.1 as i32));
                    } else if g >= neighbor.g {
                        continue;
                    }

                    let neighbor = &mut grid[neighbor_pos.0 as usize][neighbor_pos.1 as usize];
                    neighbor.g = g;
                    neighbor.h = h(current, neighbor_pos);
                    neighbor.f = neighbor.g + neighbor.h;
                    neighbor.parent = Some(current);
                }
            }
        }
    }

    println!("Overall shortest: {shortest}");
}

#[cfg(test)]
mod tests {
    use super::{add_neighbors, can_move, get_altitude, Grid, GridPoint};

    const GRID: [&str; 5] = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    fn build_grid() -> Grid {
        let mut grid: Grid = Grid::new();
        for y in 0..GRID.len() {
            let mut row: Vec<GridPoint> = Vec::new();
            let mut it = GRID[y].chars().into_iter();
            for x in 0..GRID[0].len() {
                let c = it.next().unwrap();
                row.push(GridPoint {
                    x,
                    y,
                    altitude: get_altitude(c),
                    c,
                    neighbors: Vec::new(),
                    parent: None,
                    f: 0,
                    g: 0,
                    h: 0,
                });
            }
            grid.push(row);
        }
        return grid;
    }

    #[test]
    fn test_grid() {
        let grid = build_grid();
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 8);
        assert_eq!(grid[0][0].c, 'S');
        assert_eq!(grid[3][4].c, 'u');
    }

    #[test]
    fn test_neighbors() {
        let mut grid = build_grid();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                add_neighbors(&mut grid, (y as i32, x as i32));
            }
        }
        assert_eq!(grid[0][0].neighbors.len(), 2);
        assert_eq!(grid[1][0].neighbors.len(), 3);
        assert_eq!(grid[1][1].neighbors.len(), 4);
        assert_eq!(grid[0][1].neighbors.len(), 3);
        assert_eq!(grid[3][7].neighbors.len(), 2);
    }

    #[test]
    fn test_altitude() {
        assert_eq!(get_altitude('a'), 0);
        assert_eq!(get_altitude('z'), 25);
        assert_eq!(get_altitude('e'), 4);
    }

    #[test]
    fn test_valid_moves() {
        let grid = build_grid();
        assert_eq!(can_move(&grid, (0, 0), (1, 0)), true);
        assert_eq!(can_move(&grid, (1, 0), (2, 0)), true);
        assert_eq!(can_move(&grid, (0, 1), (0, 2)), true);
        assert_eq!(can_move(&grid, (0, 2), (0, 3)), false);
        assert_eq!(can_move(&grid, (0, 2), (-1, 3)), false);
        assert_eq!(can_move(&grid, (1, 7), (0, 7)), true);
        assert_eq!(can_move(&grid, (3, 0), (4, 0)), true);
        assert_eq!(can_move(&grid, (4, 7), (3, 7)), true);
        assert_eq!(can_move(&grid, (0, 0), (grid.len() as i32, 0)), false);
        assert_eq!(can_move(&grid, (0, 0), (0, grid[0].len() as i32)), false);
    }
}
