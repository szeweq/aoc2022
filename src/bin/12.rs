/*
 * Day 12: Hill Climbing Algorithm
 * See [https://adventofcode.com/2022/day/12]
 */

use std::{collections::VecDeque};

type Pt = (usize, usize);

struct GridRange {
    u: usize,
    v: usize,
    umax: usize,
    vmax: usize
}

impl Iterator for GridRange {
    type Item = Pt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.u == self.umax && self.v == self.vmax {
            return None;
        }
        let r = (self.u, self.v);
        self.u += 1;
        if self.u == self.umax {
            self.u = 0;
            self.v += 1;
        }
        if self.v == self.vmax {
            return None;
        }
        Some(r)
    }
}

const SIDES: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn bfs(grid: &[Vec<u8>], start: Pt, goal: Pt) -> Option<usize> {
    let (w, h) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; h]; w];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some(((x, y), len)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(len);
        }
        let at = grid[x][y] + 1;
        for (dx, dy) in SIDES {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx >= w || ny >= h || visited[nx][ny] { continue; }
            let square = grid[nx][ny];
            if at >= square {
                visited[nx][ny] = true;
                queue.push_back(((nx, ny), len + 1));
            }
        }
    }
    None
}

fn range_xy(grid: &[Vec<u8>]) -> GridRange {
    GridRange { u: 0, v: 0, umax: grid.len()-1, vmax: grid[0].len()-1 }
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| Vec::from(l.as_bytes()))
        .collect::<Vec<_>>()
}

fn find_pts(grid: &[Vec<u8>]) -> Option<(Pt, Pt)> {
    let mut sp: Option<Pt> = None;
    let mut gp: Option<Pt> = None;
    for p in range_xy(grid) {
        let b = grid[p.0][p.1];
        if b == b'S' {
            sp = Some(p)
        } else if b == b'E' {
            gp = Some(p)
        }
    }
    Some((sp?, gp?))
}

pub fn part_1(input: &str) -> Option<usize> {
    let mut grid = parse_grid(input);
    let (sp, gp) = find_pts(&grid).unwrap();
    grid[sp.0][sp.1] = b'a';
    grid[gp.0][gp.1] = b'z';

    bfs(&grid, sp, gp)
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut grid = parse_grid(input);
    let (sp, gp) = find_pts(&grid).unwrap();
    grid[sp.0][sp.1] = b'a';
    grid[gp.0][gp.1] = b'z';
    let mut min = grid.len() * grid[0].len();
    for p in range_xy(&grid) {
        if grid[p.0][p.1] != b'a' { continue; }
        let Some(steps) = bfs(&grid, p, gp) else { continue; };
        if min > steps {
            min = steps;
        }
    }
    
    Some(min)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::assert_ex;

    #[test]
    fn test_part_1() {
        assert_ex!(part_1, 31);
    }

    #[test]
    fn test_part_2() {
        assert_ex!(part_2, 29);
    }
}
