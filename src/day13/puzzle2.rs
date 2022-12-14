use std::{cmp::Ordering, collections::VecDeque, fs};
pub fn run() {
    let input = fs::read_to_string("inputs/input13.txt").expect("Couldn't read file.");
    let mut lines = input.split('\n').into_iter();
    let mut packets: Vec<&str> = Vec::new();
    packets.push("[[2]]");
    packets.push("[[6]]");

    loop {
        match lines.next() {
            None => {
                break;
            }
            _ => (),
        }
        packets.push(lines.next().unwrap());
        packets.push(lines.next().unwrap());
    }

    packets.sort_by(|a, b| correct_order(a, b, 1));
    let div1 = packets
        .iter()
        .position(|p| p.cmp(&"[[2]]") == Ordering::Equal)
        .unwrap()
        + 1;
    let div2 = packets
        .iter()
        .position(|p| p.cmp(&"[[6]]") == Ordering::Equal)
        .unwrap()
        + 1;

    for p in packets {
        println!("{p}");
    }
    println!("{div1} * {div2} = {:?}", div1 * div2);
}

fn build_tokens(str: &str) -> VecDeque<&str> {
    let mut it = str.chars().into_iter();
    let mut start = 0;
    let mut end = 0;
    let mut depth = 0;
    let mut queue: VecDeque<&str> = VecDeque::new();
    loop {
        match it.next() {
            Some('[') => {
                if depth == 0 {
                    start = end + 1;
                }
                depth += 1;
            }
            Some(']') => {
                depth -= 1;
                if depth == 0 {
                    queue.push_back(&str[start..end]);
                    start = end + 1;
                }
            }
            Some(',') => {
                if depth == 1 {
                    queue.push_back(&str[start..end]);
                    start = end + 1;
                }
            }
            None => {
                break;
            }
            _ => (),
        }
        end += 1;
    }
    if start != end {
        queue.push_back(&str[start..end]);
    }
    return queue;
}

fn is_array(str: &str) -> bool {
    str.starts_with('[')
}

fn correct_order(left: &str, right: &str, depth: u32) -> Ordering {
    let mut left = build_tokens(left);
    let mut right = build_tokens(right);
    loop {
        let lval = left.pop_front();
        let rval = right.pop_front();

        // Empty left case
        if lval == None || lval == Some("") {
            if rval == None || rval == Some("") {
                return Ordering::Equal;
            }
            return Ordering::Less;
        }

        // Empty right case
        if rval == None || rval == Some("") {
            return Ordering::Greater;
        }

        let lval = lval.unwrap();
        let rval = rval.unwrap();

        println!("d: {depth}  lval: {lval}  rval: {rval}");

        if is_array(lval) && !is_array(rval) {
            let r = format!("[{}]", rval);
            let order = correct_order(lval, &r, depth + 1);
            match order {
                Ordering::Equal => {
                    continue;
                }
                x => return x,
            }
        } else if !is_array(lval) && is_array(rval) {
            let l = format!("[{}]", lval);
            let order = correct_order(&l, rval, depth + 1);
            match order {
                Ordering::Equal => {
                    continue;
                }
                x => return x,
            };
        }

        // Both values are in the same format at this point, either arrays or nums
        if !is_array(lval) {
            let l = lval.parse::<u32>().unwrap();
            let r = rval.parse::<u32>().unwrap();
            if l > r {
                return Ordering::Greater;
            } else if l == r {
                continue;
            } else {
                return Ordering::Less;
            }
        } else {
            let order = correct_order(lval, rval, depth + 1);
            if order != Ordering::Equal {
                return order;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::day13::puzzle2::correct_order;

    use super::build_tokens;

    #[test]
    fn test_build_tokens_simple() {
        let left = "[1,2,3,4,5]";
        let mut queue = build_tokens(left);

        assert_eq!(queue.pop_front(), Some("1"));
        assert_eq!(queue.pop_front(), Some("2"));
        assert_eq!(queue.pop_front(), Some("3"));
        assert_eq!(queue.pop_front(), Some("4"));
        assert_eq!(queue.pop_front(), Some("5"));
        assert_eq!(queue.pop_front(), None);
    }

    #[test]
    fn test_build_tokens_array() {
        let left = "[[1],2,3]";
        let mut queue = build_tokens(left);

        assert_eq!(queue.pop_front(), Some("[1]"));
        assert_eq!(queue.pop_front(), Some("2"));
        assert_eq!(queue.pop_front(), Some("3"));
        assert_eq!(queue.pop_front(), None);
    }

    #[test]
    fn test_build_tokens_array2() {
        let left = "[[],[2,3,4],[[1],[]]]";
        let mut queue = build_tokens(left);

        assert_eq!(queue.pop_front(), Some("[]"));
        assert_eq!(queue.pop_front(), Some("[2,3,4]"));
        assert_eq!(queue.pop_front(), Some("[[1],[]]"));
        assert_eq!(queue.pop_front(), None);
    }

    #[test]
    fn test_build_tokens_empty() {
        let left = "[]";
        let mut queue = build_tokens(left);
        assert_eq!(queue.pop_front(), Some(""));
    }

    #[test]
    fn test_order_numeric() {
        let left = "[1,2,3,4]";
        let right = "[2,3,4,5]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_numeric2() {
        let left = "[2,3,4,5]";
        let right = "[1,2,3,4]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_numeric3() {
        let left = "[1,2]";
        let right = "[1,2,3]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_numeric4() {
        let left = "[1,1,1]";
        let right = "[1,1,2]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_empty() {
        let left = "[]";
        let right = "[1,1,2]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_empty2() {
        let left = "[]";
        let right = "[]";
        assert_eq!(correct_order(left, right, 1), Ordering::Equal);
    }

    #[test]
    fn test_order_empty3() {
        let left = "[1]";
        let right = "[]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_array() {
        let left = "[1,[2],3]";
        let right = "[2,3,4]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_array2() {
        let left = "[[1],[2,3,4]]";
        let right = "[[1],4]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_array3() {
        let left = "[9]";
        let right = "[[8,7,6]]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_array4() {
        let left = "[[]]";
        let right = "[[[]]]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_array5() {
        let left = "[[[[[]]]]]";
        let right = "[[[[]]]]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_array6() {
        let left = "[[3,5,10]]";
        let right = "[1]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_array7() {
        let left = "[1,1,3]";
        let right = "[1,2,1]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_array8() {
        let left = "[[2,9]]";
        let right = "[2]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_array9() {
        let left = "[1,[2],[3,4,5],6]";
        let right = "[1,2,[3,4,[5]],10]";
        assert_eq!(correct_order(left, right, 1), Ordering::Less);
    }

    #[test]
    fn test_order_array11() {
        let left = "[[4,4],4,4,4]";
        let right = "[[4,4],4,4]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }

    #[test]
    fn test_order_array12() {
        let left = "[[4,4],4,4,4]";
        let right = "[[4,4],4]";
        assert_eq!(correct_order(left, right, 1), Ordering::Greater);
    }
}
