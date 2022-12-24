/*
 * Day 24: Blizzard Basin
 * See [https://adventofcode.com/2022/day/24]
 */

use std::collections::HashSet;

const MOVES: [(isize, isize); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut grid = vec![];
    for (i, l) in input.lines().enumerate() {
        let bl = l.as_bytes();
        if i > 0 && bl[1] != b'#' {
            let v = bl[1..l.len()-1].iter().map(|&x| {
                match x {
                    b'.' => 0,
                    b'>' => 1,
                    b'v' => 2,
                    b'<' => 4,
                    b'^' => 8,
                    _ => panic!("Invalid character: {}", x as char)
                }
            }).collect::<Vec<_>>();
            grid.push(v);
        }
    }
    grid
}

fn clear_at(grid: &Vec<Vec<u8>>, p: (usize, usize), t: usize) -> bool {
    let (x, y) = p;
    let xl = grid[y].len();
    let yl = grid.len();
    let xt = t % xl;
    let yt = t % yl;
    let lu = grid[(y+yl-yt)%yl][x] & 2;
    let ld = grid[(y+yt)%yl][x] & 8;
    let ll = grid[y][(x+xl-xt)%xl] & 1;
    let lr = grid[y][(x+xt)%xl] & 4;
    (lu | ld | ll | lr) == 0
}

fn travel(grid: &Vec<Vec<u8>>, from: (usize, usize), to: (usize, usize), time: usize) -> usize {
    let mut pos_hs = HashSet::new();
    pos_hs.insert(from);
    let mut upos = HashSet::new();
    let mut t = time;
    let size = (grid[0].len(), grid.len());
    while !clear_at(grid, from, t) {
        t += 1;
    }
    'l1: loop {
        t += 1;
        for p in &pos_hs {
            for m in MOVES {
                let dp = (p.0.wrapping_add_signed(m.0), p.1.wrapping_add_signed(m.1));
                if dp.0 < size.0 && dp.1 < size.1 {
                    if clear_at(&grid, dp, t) {
                        if dp == to {
                            break 'l1;
                        }
                        upos.insert(dp);
                    }
                }
            }
        }
        if !upos.is_empty() {
            pos_hs.clear();
            pos_hs.extend(&upos);
            upos.clear();
        }
    }
    t + 1
}

pub fn part_1(input: &str) -> Option<usize> {
    let grid = parse(input);
    Some(travel(&grid, (0, 0), (grid[0].len() - 1, grid.len() - 1), 0))
}

pub fn part_2(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = (0, 0);
    let finish = (grid[0].len() - 1, grid.len() - 1);
    let mut time = travel(&grid, start, finish, 0);
    time = travel(&grid, finish, start, time + 1);
    time = travel(&grid, start, finish, time + 1);
    Some(time)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 18); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 54); }
}