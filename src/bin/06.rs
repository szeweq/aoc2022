use std::collections::{HashMap};

pub fn part_1(input: &str) -> Option<u32> {
    let b = input.as_bytes();
    let mut sol = 0;
    for x in 3..b.len() {
        let (a, b, c, d) = (b[x-3], b[x-2], b[x-1], b[x]);
        if a == b || a == c || a == d || b == c || b == d || c == d {
            continue;
        }
        sol = x as u32 + 1;
        break;
    }
    Some(sol)
}

pub fn part_2(input: &str) -> Option<u32> {
    let b = input.as_bytes();
    let mut sol = 0;
    let mut hm: HashMap<u8, usize> = HashMap::with_capacity(15);
    let mut x = 13;

    'l: while x < b.len() {
        hm.clear();
        for i in 0..14 {
            let val = b[x-13+i];
            match hm.insert(val, i) {
                Some(m) => {
                    x += 1 + m;
                    continue 'l;
                }
                None => {}
            }
        }
        sol = x as u32 + 1;
        break;
    }
    Some(sol)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 7); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 19); }
}