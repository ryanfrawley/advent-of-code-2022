use std::{
    fs,
    io::{stdin, stdout, Read, Write},
    vec,
};

type Point = (i32, i32);
type Bounds = (Point, Point);

fn parse_point(p: &str) -> Point {
    let index = p.chars().position(|c| c == ',').unwrap();
    (
        p[(index + 1)..].parse::<i32>().unwrap(),
        p[0..index].parse::<i32>().unwrap(),
    )
}

fn intersect_point(p: &Point, bounds: &Bounds) -> bool {
    p.0 >= bounds.0 .0 && p.0 <= bounds.1 .0 && p.1 >= bounds.0 .1 && p.1 <= bounds.1 .1
}

fn cave_to_grid(p: &Point, bounds: &Bounds) -> Option<Point> {
    match intersect_point(p, bounds) {
        true => Some((p.0 - bounds.0 .0, p.1 - bounds.0 .1)),
        false => None,
    }
}

fn print_cave(cave: &Vec<Vec<char>>) {
    for y in cave {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
}

fn normalize(n: i32) -> i32 {
    match n.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn cave_location_mut<'a>(
    cave: &'a mut Vec<Vec<char>>,
    p: &Point,
    bounds: &Bounds,
) -> Option<&'a mut char> {
    let p = cave_to_grid(p, bounds);
    match p {
        Some(p) => Some(&mut cave[p.0 as usize][p.1 as usize]),
        None => None,
    }
}

const SAND_SPAWN: Point = (0, 500);

pub fn run() {
    let input = fs::read_to_string("inputs/input14.txt").expect("Couldn't read file.");
    let lines = input.split('\n').into_iter();
    let mut min: Point = SAND_SPAWN;
    let mut max: Point = SAND_SPAWN;
    let mut paths: Vec<Vec<Point>> = Vec::new();
    for line in lines {
        let mut tokens = line.split(" -> ").into_iter();
        let mut path: Vec<Point> = Vec::new();
        while let Some(p) = tokens.next() {
            let point = parse_point(p);

            // Update y min/max
            if point.0 < min.0 {
                min.0 = point.0;
            } else if point.0 > max.0 {
                max.0 = point.0;
            }

            // Update x min/max
            if point.1 < min.1 {
                min.1 = point.1;
            } else if point.1 > max.1 {
                max.1 = point.1;
            }

            path.push(point);
            println!("({}, {})", point.0, point.1);
        }

        paths.push(path);
    }

    // PT II.
    // Add floor
    max.0 += 2;
    min.1 = 0;
    max.1 = 1000;
    let mut floor: Vec<Point> = Vec::new();
    floor.push((max.0, min.1));
    floor.push((max.0, max.1));
    paths.push(floor);

    println!("Min: ({},{}) - Max: ({},{})", min.0, min.1, max.0, max.1);

    // Build the cave map
    let mut cave: Vec<Vec<char>> = Vec::with_capacity((max.0 - min.0 + 1).try_into().unwrap());
    for y in 0..cave.capacity() {
        let mut row: Vec<char> = Vec::with_capacity((max.1 - min.1 + 1).try_into().unwrap());
        for x in 0..row.capacity() {
            row.push('.');
        }
        cave.push(row);
    }

    // Generate rocks
    for path in paths {
        println!("=== New path");
        let mut it = path.iter();
        let mut start = it.next().unwrap();
        while let Some(end) = it.next() {
            println!("Start: ({},{})", start.0, start.1);
            println!("End: ({},{})", end.0, end.1);
            let vector = (end.0 - start.0, end.1 - start.1);
            println!("Vector: ({},{})", vector.0, vector.1);
            let dy = normalize(vector.0);
            let dx = normalize(vector.1);
            println!("Normalized: ({},{})", dy, dx);
            assert!(dx == 0 || dy == 0); // We can only move horizontally OR vertically; not both
            let mut current = *start;
            loop {
                println!("Current: ({},{})", current.0, current.1);
                let vec_coord = cave_to_grid(&current, &(min, max)).unwrap();
                cave[vec_coord.0 as usize][vec_coord.1 as usize] = '#';

                if current.0 == end.0 && current.1 == end.1 {
                    break;
                }
                current.0 += dy;
                current.1 += dx;
            }

            start = end;
        }
    }

    let sand_vectors: Vec<Point> = vec![(1, 0), (1, -1), (1, 1)];

    // Simulate sand
    let mut sand: Point = SAND_SPAWN;
    loop {
        let mut moved = false;
        let mut out_of_bounds = true;

        let start_obj = cave_location_mut(&mut cave, &sand, &(min, max)).unwrap();
        match *start_obj {
            'o' | '#' => {
                break;
            }
            _ => (),
        };

        for v in &sand_vectors {
            let next_sand = (sand.0 + v.0, sand.1 + v.1);
            println!("Trying next_sand: ({},{})", next_sand.0, next_sand.1);

            let next_obj = cave_location_mut(&mut cave, &next_sand, &(min, max));
            match next_obj {
                Some('.') => {
                    sand = next_sand;
                    moved = true;
                    out_of_bounds = false;
                    break;
                }
                Some('#') | Some('o') => {
                    out_of_bounds = false;
                    continue;
                }
                None => {
                    out_of_bounds = true;
                    break;
                }
                _ => {
                    panic!("Invalid char encountered");
                }
            }
        }

        if out_of_bounds {
            break;
        }

        // Sand hasn't moved this step, so settle in this location
        if !moved {
            let settle_pos = cave_location_mut(&mut cave, &sand, &(min, max)).unwrap();
            *settle_pos = 'o';
            sand = SAND_SPAWN;
        }
    }

    print_cave(&cave);

    // Count the result
    let mut result = 0;
    for y in cave {
        for x in y {
            if x == 'o' {
                result += 1;
            }
        }
    }

    println!("Grains: {}", result);
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
