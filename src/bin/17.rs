/*
 * Day 17: Pyroclastic Flow
 * See [https://adventofcode.com/2022/day/17]
 */

use std::collections::HashMap;

const B_W: [u8; 5] = [4, 3, 3, 1, 2];

const BCH: [for<'r> fn(&'r Vec<u8>, u8, usize) -> bool; 5] = [b0_check, b1_check, b2_check, b3_check, b4_check];
const BFL: [for<'r> fn(&'r mut Vec<u8>, (u8, usize)); 5] = [b0_fill, b1_fill, b2_fill, b3_fill, b4_fill];

fn b0_check(v: &Vec<u8>, x: u8, y: usize) -> bool {
    let c = 15u8 << x;
    v[y] & c == 0
}
fn b1_check(v: &Vec<u8>, x: u8, y: usize) -> bool {
    let c1 = 2 << x;
    let c2 = 7 << x;
    v[y] & c1 == 0 && v[y + 1] & c2 == 0 && v[y + 2] & c1 == 0
}
fn b2_check(v: &Vec<u8>, x: u8, y: usize) -> bool {
    let c1 = 7 << x;
    let c2 = 1 << x;
    v[y] & c1 == 0 && v[y + 1] & c2 == 0 && v[y + 2] & c2 == 0
}
fn b3_check(v: &Vec<u8>, x: u8, y: usize) -> bool {
    let c = 1 << x;
    v[y] & c == 0 && v[y + 1] & c == 0 && v[y + 2] & c == 0 && v[y + 3] & c == 0
}
fn b4_check(v: &Vec<u8>, x: u8, y: usize) -> bool {
    let c = 3 << x;
    v[y] & c == 0 && v[y + 1] & c == 0
}

fn b0_fill(v: &mut Vec<u8>, p: (u8, usize)) {
    v[p.1] |= 15 << p.0;
}
fn b1_fill(v: &mut Vec<u8>, p: (u8, usize)) {
    let c1 = 2 << p.0;
    let c2 = 7 << p.0;
    v[p.1] |= c1;
    v[p.1 + 1] |= c2;
    v[p.1 + 2] |= c1;
}
fn b2_fill(v: &mut Vec<u8>, p: (u8, usize)) {
    let c1 = 7 << p.0;
    let c2 = 1 << p.0;
    v[p.1] |= c1;
    v[p.1 + 1] |= c2;
    v[p.1 + 2] |= c2;
}
fn b3_fill(v: &mut Vec<u8>, p: (u8, usize)) {
    let c = 1 << p.0;
    v[p.1] |= c;
    v[p.1 + 1] |= c;
    v[p.1 + 2] |= c;
    v[p.1 + 3] |= c;
}
fn b4_fill(v: &mut Vec<u8>, p: (u8, usize)) {
    let c = 3 << p.0;
    v[p.1] |= c;
    v[p.1 + 1] |= c;
}

fn row_cut(v: &Vec<u8>) -> Option<usize> {
    let mut max = 0;
    for (yy, xx) in v.iter().enumerate().rev() {
        max |= *xx;
        if max == 127 {
            return Some(yy);
        }
    }
    None
}

fn view_blocks(v: &Vec<u8>, bl: u8, i: usize) -> ([u8; 7], u8, usize) {
    let mut hx = [0; 7];
    for (yy, xx) in v.iter().enumerate().rev() {
        let ux = *xx;
        if ux == 0 { continue; }
        for j in 0..7 {
            if ux & 1 << j != 0 {
                if hx[j] == 0 {
                    hx[j] = yy as u8;
                }
            }
        }
    }
    let &hmin = hx.iter().min().unwrap();
    for j in 0..7 {
        hx[j] -= hmin;
    }
    (hx, bl, i)
}

fn find_top(v: &Vec<u8>) -> usize {
    v.iter().position(|&x| x == 0).unwrap()
}

fn solve(bi: &[u8], max_rocks: usize) -> usize {
    // MANUALLY SET SIZE
    let mut v: Vec<u8> = vec![0; 96];
    let mut hs = HashMap::new();
    let mut pos = (1, 3);
    let mut h = 0;
    let mut rocks = 0;
    let mut i = 0;
    while rocks < max_rocks {
        let block = rocks % 5;
        let b = bi[i];
        match &b {
            b'>' => {
                if pos.0 > 0 {
                    if BCH[block](&v, pos.0-1, pos.1) {
                        pos.0 -= 1;
                    }
                }
            }
            b'<' => {
                if pos.0 + B_W[block] < 7 {
                    if BCH[block](&v, pos.0+1, pos.1) {
                        pos.0 += 1;
                    }
                }
            }
            _ => {}
        }
        let (x, y) = pos;
        if pos.1 > 0 && BCH[block](&v, x, y-1) {
            pos.1 -= 1;
        } else {
            BFL[block](&mut v, pos);
            rocks += 1;
            if let Some(yc) = row_cut(&v) {
                if yc > 1 {
                    h += yc - 1;
                    for iy in 0..v.len() {
                        let yy = yc + iy - 1;
                        v[iy] = if yy < v.len() { v[yy] } else { 0 };
                    }
                }
            }
            let vt = find_top(&v);
            pos.0 = 5 - B_W[rocks % 5];
            pos.1 = vt + 3;
            let vbz = view_blocks(&v, block as u8, i);
            if let Some((r, ty)) = hs.get(&vbz) {
                let rep = (max_rocks - rocks) / (rocks - r);
                let extra = (rocks - r) * rep;
                let plus = (h + vt - ty) * rep;
                rocks += extra;
                h += plus;
                hs.clear();
            }
            hs.insert(vbz, (rocks, h + vt));
        }
        i = (i + 1) % bi.len();
    }
    h + find_top(&v)
}

pub fn part_1(input: &str) -> Option<usize> {
    Some(solve(input.as_bytes(), 2022))
}

pub fn part_2(input: &str) -> Option<usize> {
    Some(solve(input.as_bytes(), 1000000000000))
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 3068); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 1514285714288); }
}