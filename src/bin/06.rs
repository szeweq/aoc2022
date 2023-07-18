/*
 * Day 6: Tuning Trouble
 * See [https://adventofcode.com/2022/day/6]
 */

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
    let mut fbits = 0u32;
    b.iter().take(13).for_each(|c| fbits ^= 1 << (c % 32));
    b.windows(14).position(|w| {
        let fst = w[0];
        let lst = w[w.len() - 1];
        fbits ^= 1 << (lst % 32);
        let res = fbits.count_ones() == 14;
        fbits ^= 1 << (fst % 32);
        res
    }).map(|x| (x + 14) as u32)
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