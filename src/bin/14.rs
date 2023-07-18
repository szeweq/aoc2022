/*
 * Day 14: Regolith Reservoir
 * See [https://adventofcode.com/2022/day/14]
 */

use std::collections::HashSet;

fn parse_path(input: &str) -> Vec<Vec<(u32, u32)>> {
    input.lines().map(|l| {
        l.split(" -> ").map(|pp| {
            let (px, py) = pp.split_once(',').unwrap();
            (px.parse::<u32>().unwrap(), py.parse::<u32>().unwrap())
        }).collect()
    }).collect()
}

fn _insert(v: &mut Vec<HashSet<u32>>, x: u32, y: u32) {
    if (y as usize) >= v.len() {
        v.resize_with((y as usize) + 1, HashSet::new);
    }
    v[y as usize].insert(x);
}

fn parse_map(v: Vec<Vec<(u32, u32)>>) -> Vec<HashSet<u32>> {
    let mut vh = Vec::new();

    for p in v {
        let mut xp = p[0];
        for &cp in &p[1..] {
            let x1 = xp.0.min(cp.0);
            let x2 = xp.0.max(cp.0);
            let y1 = xp.1.min(cp.1) as usize;
            let y2 = xp.1.max(cp.1) as usize;
            if x1 == x2 {
                if y2 >= vh.len() {
                    vh.resize_with(y2 + 1, HashSet::new);
                }
                for hs in &mut vh[y1..=y2] {
                    hs.insert(x1);
                }
            } else if y1 == y2 {
                if y1 >= vh.len() {
                    vh.resize_with(y1 + 1, HashSet::new);
                }
                vh[y1].extend(x1..=x2);
            }
            xp = cp;
        }
    }
    vh
}

pub fn part_1(input: &str) -> Option<u32> {
    let v = parse_path(input);
    let mut m = parse_map(v);
    let ly = m.len();
    let spos: (u32, usize) = (500, 0);
    let mut i = 0u32;
    let mut route = vec![spos];
    while let Some(&cpos) = route.last() {
        if cpos.1 >= ly {
            break;
        }
        let mut dx = cpos.0;
        let dy = cpos.1 + 1;
        if let Some(h) = m.get(dy) {
            if h.contains(&dx) {
                dx -= 1;
                if h.contains(&dx) {
                    dx += 2;
                    if h.contains(&dx) {
                        m[cpos.1].insert(cpos.0);
                        route.pop();
                        i += 1;
                        continue;
                    }
                }
            }
        }
        route.push((dx, dy));
    }
    Some(i)
}

pub fn part_2(input: &str) -> Option<u32> {
    let v = parse_path(input);
    let mut m = parse_map(v);
    m.push(HashSet::new());
    let ly = m.len() - 1;
    let mut i = 0u32;
    let mut route = vec![(500, 0usize)];
    while let Some(&cpos) = route.last() {
        if cpos.1 == ly {
            m[cpos.1].insert(cpos.0);
            route.pop();
            i += 1;
            continue;
        }
        let mut dx = cpos.0;
        let dy = cpos.1 + 1;
        if let Some(h) = m.get(dy) {
            if h.contains(&dx) {
                dx -= 1;
                if h.contains(&dx) {
                    dx += 2;
                    if h.contains(&dx) {
                        if cpos == (500, 0usize) {
                            break;
                        }
                        m[cpos.1].insert(cpos.0);
                        route.pop();
                        i += 1;
                        continue;
                    }
                }
            }
        }
        route.push((dx, dy));
    }
    Some(i + 1)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 24); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 93); }
}