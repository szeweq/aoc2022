/*
 * Day 20: Grove Positioning System
 * See [https://adventofcode.com/2022/day/20]
 */
use std::collections::VecDeque;

fn solve<T>(nums: T, loops: usize) -> isize where T: Iterator<Item = isize> {
    let mut en = nums.enumerate().collect::<VecDeque<_>>();
    let il = en.len() as isize;
    for _ in 0..loops {
        for a in 0..en.len() {
            let pos = en.iter().position(|(b, _)| *b == a).unwrap();
            en.rotate_left(pos);
            let m = en.pop_front().unwrap();
            let ix = m.1.rem_euclid(il - 1) as usize;
            en.rotate_left(ix);
            en.push_back(m);
        }
    }
    let l = en.len();
    let pos = en.iter().position(|(_, x)| *x == 0).unwrap();
    [1000, 2000, 3000].iter().map(|&i| en[(pos + i) % l].1).sum()
}

fn must_parse(l: &str) -> isize {
    l.parse().unwrap()
}

pub fn part_1(input: &str) -> Option<isize> {
    let nums = input.lines().map(must_parse);
    Some(solve(nums, 1))
}

pub fn part_2(input: &str) -> Option<isize> {
    let nums = input.lines().map(must_parse).map(|x| x * 811589153);
    Some(solve(nums, 10))
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 3); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 1623178306); }
}