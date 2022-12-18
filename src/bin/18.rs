/*
 * Day 18: Boiling Boulders
 * See [https://adventofcode.com/2022/day/18]
 */
use std::{collections::HashSet, ops::RangeInclusive};


fn parse_line(l: &str) -> (i32, i32, i32) {
    let mut it = l.split(',').map(|x| x.parse::<i32>().unwrap());
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}

fn pt_neighbors(x: i32, y: i32, z: i32) -> [(i32, i32, i32); 6] {
    [
        (x + 1, y, z), (x - 1, y, z),
        (x, y + 1, z), (x, y - 1, z),
        (x, y, z + 1), (x, y, z - 1)
    ]
}

pub fn part_1(input: &str) -> Option<usize> {
    let pts = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut cbs = HashSet::new();
    for i in 0..pts.len() {
        let (x1, y1, z1) = pts[i];
        for j in i..pts.len() {
            let (x2, y2, z2) = pts[j];
            let a = match (x1 - x2, y1 - y2, z1 - z2) {
                (1, 0, 0) => 0,
                (0, 1, 0) => 1,
                (0, 0, 1) => 2,
                (-1, 0, 0) => 3,
                (0, -1, 0) => 4,
                (0, 0, -1) => 5,
                _ => continue
            };
            cbs.insert((x1, y1, z1, a));
            cbs.insert((x2, y2, z2, (a + 3) % 6));
        }
    }
    Some(pts.len() * 6 - cbs.len())
}

// MANUALLY SET VALUES
const PT_RANGE: RangeInclusive<i32> = -1..=20;

pub fn part_2(input: &str) -> Option<u32> {
    let ptset = input.lines().map(parse_line).collect::<HashSet<_>>();
    let mut chpt = HashSet::from([(20, 20, 20)]);
    let mut update = HashSet::new();
    loop {
        update.clear();
        for cp in chpt.iter() {
            let (x, y, z) = *cp;
            for np in &pt_neighbors(x, y, z) {
                if !ptset.contains(np) && !chpt.contains(np) {
                    let (nx, ny, nz) = *np;
                    if PT_RANGE.contains(&nx)
                    && PT_RANGE.contains(&ny)
                    && PT_RANGE.contains(&nz) {
                        update.insert(*np);
                    }
                }
            }
        }
        if update.is_empty() {
            break;
        }
        chpt.extend(&update);
    }
    let mut cnt = 0;
    for p in &ptset {
        let (x, y, z) = *p;
        for np in &pt_neighbors(x, y, z) {
            if chpt.contains(np) {
                cnt += 1;
            }
        }
    }
    println!("PTS: {}, CH: {}", ptset.len()*6, chpt.len());
    Some(cnt)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 64); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 58); }
}