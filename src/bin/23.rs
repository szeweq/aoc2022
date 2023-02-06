/*
 * Day 23: Unstable Diffusion
 * See [https://adventofcode.com/2022/day/23]
 */
use std::collections::{HashMap, HashSet};

type Pt = (isize, isize);


fn parse_map(input: &str) -> HashSet<Pt> {
    input.lines().enumerate().flat_map(|(i, l)| {
        l.bytes().enumerate().filter_map(move |(j, b)| {
            if b == b'#' { Some((i as isize, j as isize)) } else { None }
        })
    }).collect()
}

const LOOKUP: [Pt; 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
const MOVES: [Pt; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const CHECKS: [usize; 4] = [7, 224, 41, 148];

fn update_elf_pos(hm: &mut HashSet<Pt>, dir: usize) -> usize {
    let mut moved = 0;
    let keys = hm.iter().copied().collect::<Vec<_>>();
    let mut prm = HashMap::new();
    for j in 0..keys.len() {
        let elf = keys[j];
        let mut adj = 0;
        for a in 0..8 {
            let f = LOOKUP[a];
            if !hm.contains(&(elf.0 + f.0, elf.1 + f.1)) {
                adj |= 1 << a;
            }
        }
        if adj == 255 {
            continue;
        }

        for i in 0..4 {
            let x = (dir + i) % 4;
            let ch = CHECKS[x];
            if adj & ch == ch {
                let f = MOVES[x];
                let npos = (elf.0 + f.0, elf.1 + f.1);
                match prm.get_mut(&npos) {
                    Some(x) if *x < keys.len() => { *x = keys.len(); },
                    None => { prm.insert(npos, j); },
                    _ => {}
                }
                break;
            }
        }
    }
    for (np, ei) in prm {
        if ei < keys.len() {
            let elf = &keys[ei];
            hm.remove(elf);
            hm.insert(np);
            moved += 1;
        }
    }
    moved
}

pub fn part_1(input: &str) -> Option<isize> {
    let mut hm = parse_map(input);
    for r in 0..10 {
        update_elf_pos(&mut hm, r);
    }
    let l = hm.len() as isize;
    let mut itr = hm.into_iter();
    let fst = itr.next().unwrap();
    let (mut hmin, mut hmax) = (fst.0, fst.0);
    let (mut wmin, mut wmax) = (fst.1, fst.1);

    for (r, c) in itr {
        if hmin > r { hmin = r; }
        if hmax < r { hmax = r; }
        if wmin > c { wmin = c; }
        if wmax < c { wmax = c; }
    }
    let h = hmax - hmin + 1;
    let w = wmax - wmin + 1;
    Some(h * w - l)
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut hm = parse_map(input);
    let mut r = 0;
    let mut n = 1;
    while n > 0 {
        n = update_elf_pos(&mut hm, r);
        r += 1;
    }
    Some(r)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 110); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 20); }
}