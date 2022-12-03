use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input2-1.txt").expect("Couldn't read file.");
    let mut score = 0;
    for token in input.split('\n') {
        println!("{score}");
        let chars = token.as_bytes();
        let opponent = chars[0] as char;
        let outcome = chars[2] as char;

        let mine: char;
        if outcome == 'X' {
            // Lose
            if opponent == 'A' {
                mine = 'Z';
            } else if opponent == 'B' {
                mine = 'X';
            } else {
                mine = 'Y';
            }
        } else if outcome == 'Y' {
            // Draw
            if opponent == 'A' {
                mine = 'X';
            } else if opponent == 'B' {
                mine = 'Y';
            } else {
                mine = 'Z';
            }
        } else {
            // Win
            if opponent == 'A' {
                mine = 'Y';
            } else if opponent == 'B' {
                mine = 'Z';
            } else {
                mine = 'X';
            }
        }

        if mine == 'X' {
            score += 1;
        } else if mine == 'Y' {
            score += 2;
        } else if mine == 'Z' {
            score += 3;
        }

        // Draw
        if (mine == 'X' && opponent == 'A')
            || (mine == 'Y' && opponent == 'B')
            || (mine == 'Z' && opponent == 'C')
        {
            score += 3;
            continue;
        }

        // Win
        if (mine == 'X' && opponent == 'C')
            || (mine == 'Y' && opponent == 'A')
            || (mine == 'Z' && opponent == 'B')
        {
            score += 6;
        }
    }
    println!("Score: {score}");
}
