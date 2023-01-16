/*
 * Day 15: Beacon Exclusion Zone
 * See [https://adventofcode.com/2022/day/15]
 */

use std::collections::HashSet;

fn parse_sensors(input: &str) -> Vec<((i32, i32), (i32, i32), i32)> {
    input.lines().map(|l| {
        let (a, b) = l.split_once(':').unwrap();
        let (aa, ab) = a.split_once(',').unwrap();
        let (ba, bb) = b.split_once(',').unwrap();
        let x1: i32 = aa[12..].parse().unwrap();
        let y1: i32 = ab[3..].parse().unwrap();
        let x2 = ba[24..].parse().unwrap();
        let y2 = bb[3..].parse().unwrap();
        ((x1, y1), (x2, y2), ((x1.abs_diff(x2)) + (y1.abs_diff(y2))) as i32)
    }).collect::<Vec<_>>()
}

pub fn part_1(input: &str) -> Option<usize> {
    let vals = parse_sensors(input);
    let chy = if cfg!(test) { 10 } else { 2000000 };
    let mut xsum = 0;
    let mut chx = HashSet::new();
    for (s, b, st) in vals {
        if b.1 == chy {
            chx.insert(b.0);
        }
        let my = s.1.abs_diff(chy) as i32;
        if st > my {
            let cs = st - my;
            let cx1 = s.0 - cs;
            let cx2 = s.0 + cs;
            for xc in cx1..=cx2 {
                if !chx.contains(&xc) {
                    xsum += 1;
                    chx.insert(xc);
                }
            }
        }
    }
    Some(xsum)
}

pub fn part_2(input: &str) -> Option<i64> {
    let vals = parse_sensors(input);
    let zq = if cfg!(test) { 20 } else { 4000000 };
    for y in 0..=zq {
        let mut ranges = Vec::new();
        for ((sx, sy), _, st) in &vals {
            let my = sy.abs_diff(y) as i32;
            let cs = st - my;
            if cs < 0 {
                continue;
            }
            ranges.push((sx-cs, sx+cs));
        }
        ranges.sort();
        let mut comp = Vec::new();
        let (mut plox, mut phix) = ranges[0];
        for p in &ranges[1..] {
            let (lox, hix) = *p;
            if lox-1 <= phix {
                phix = phix.max(hix)
            } else {
                comp.push((plox, phix));
                plox = lox;
                phix = hix;
            }
        }

        if comp.len() == 1 {
            let x = (comp[0].1 + 1) as i64;
            return Some(x * 4000000 + (y as i64));
        }
    }
    None
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 26); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 56000011); }
}