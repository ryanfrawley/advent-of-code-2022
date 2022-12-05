use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input5.txt").expect("Couldn't read file.");
    let lines = input.split('\n');
    let mut stacks = vec![
        vec!['Q', 'S', 'W', 'C', 'Z', 'V', 'F', 'T'],
        vec!['Q', 'R', 'B'],
        vec!['B', 'Z', 'T', 'Q', 'P', 'M', 'S'],
        vec!['D', 'V', 'F', 'R', 'Q', 'H'],
        vec!['J', 'G', 'L', 'D', 'B', 'S', 'T', 'P'],
        vec!['W', 'R', 'T', 'Z'],
        vec!['H', 'Q', 'M', 'N', 'S', 'F', 'R', 'J'],
        vec!['R', 'N', 'F', 'H', 'W'],
        vec!['J', 'Z', 'T', 'Q', 'P', 'R', 'B'],
    ];
    for l in lines {
        let mut it = l.split(' ').into_iter();
        it.next();
        let count = it.next().unwrap().parse::<i32>().unwrap();
        it.next();
        let from = it.next().unwrap().parse::<i32>().unwrap();
        it.next();
        let to = it.next().unwrap().parse::<i32>().unwrap();
        let mut i = 0;
        let mut crane_stack = Vec::new();
        while i < count {
            let c = stacks[(from - 1) as usize].pop().unwrap();
            crane_stack.push(c);
            i += 1;
        }
        i = 0;
        while i < count {
            let c = crane_stack.pop().unwrap();
            stacks[(to - 1) as usize].push(c);
            i += 1;
        }
    }
    for stack in stacks {
        let c = stack.last().unwrap();
        print!("{c}");
    }
    println!("");
}
