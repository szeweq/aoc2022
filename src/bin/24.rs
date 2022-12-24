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

fn air_at(grid: &Vec<Vec<u8>>, x: usize, y: usize, t: usize) -> u8 {
    let xl = grid[y].len();
    let yl = grid.len();
    let xt = t % xl;
    let yt = t % yl;
    let lu = grid[(y+yl-yt)%yl][x] & 2;
    let ld = grid[(y+yt)%yl][x] & 8;
    let ll = grid[y][(x+xl-xt)%xl] & 1;
    let lr = grid[y][(x+xt)%xl] & 4;
    lu | ld | ll | lr
}

fn travel(grid: &Vec<Vec<u8>>, from: (usize, usize), to: (usize, usize), time: usize) -> usize {
    let mut pos_hs = HashSet::new();
    pos_hs.insert(from);
    let mut t = time;
    let size = (grid[0].len(), grid.len());
    while air_at(grid, from.0, from.1, t) != 0 {
        t += 1;
    }
    while !pos_hs.contains(&to) {
        t += 1;
        let mut npos_hs = HashSet::new();
        for p in &pos_hs {
            for m in MOVES {
                let dp = (p.0.wrapping_add_signed(m.0), p.1.wrapping_add_signed(m.1));
                if dp.0 < size.0 && dp.1 < size.1 {
                    let at = air_at(&grid, dp.0, dp.1, t);
                    if at == 0 {
                        npos_hs.insert(dp);
                    }
                }
            }
        }
        if !npos_hs.is_empty() {
            pos_hs.clear();
            pos_hs.extend(npos_hs);
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