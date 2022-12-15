/*
 * Day 9: Rope Bridge
 * See [https://adventofcode.com/2022/day/9]
 */

use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pt {
    x: i32,
    y: i32,
}

macro_rules! pt {
    [$x: expr, $y: expr] => { Pt { x: $x, y: $y } };
    [] => { Pt { x: 0, y: 0 } };
}

enum Side {
    X, Y
}

fn parse_step(l: &str) -> (Side, i32) {
    use Side::*;
    let n: i32 = l[2..].parse().unwrap();
    match l.as_bytes()[0] {
        b'U' => (Y, n),
        b'D' => (Y, -n),
        b'L' => (X, -n),
        b'R' => (X, n),
        _ => panic!("Unknown direction"),
    }
}

pub fn part_1(input: &str) -> Option<usize> {
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;
    let mut paths: HashSet<Pt> = HashSet::new();
    paths.insert(pt![]);
    for l in input.lines() {
        let (side, st) = parse_step(l);
        match side {
            Side::Y => {
                hy += st;
                let d = hy.abs_diff(ty);
                if d > 1 {
                    tx = hx;
                    let p = st.signum();
                    for _ in 1..d {
                        ty += p;
                        paths.insert(pt![tx, ty]);
                    }
                }
            }
            Side::X => {
                hx += st;
                let d = hx.abs_diff(tx);
                if d > 1 {
                    ty = hy;
                    let p = st.signum();
                    for _ in 1..d {
                        tx += p;
                        paths.insert(pt![tx, ty]);
                    }
                }
            }
        }
    }
    Some(paths.len())
}

#[inline]
fn abs_d(p1: &Pt, p2: &Pt) -> u8 {
    if p1.x.abs_diff(p2.x) > 1 {
        if p1.y != p2.y {
            2
        } else {
            1
        }
    } else if p1.y.abs_diff(p2.y) > 1 {
        if p1.x != p2.x {
            2
        } else {
            1
        }
    } else {
        0
    }
}

macro_rules! pt_cmp {
    ($xp: expr, $cp: expr) => {
        if $xp > $cp {
            $cp + 1
        } else {
            $cp - 1
        }
    };
}

#[inline]
fn adv(pos: &mut [Pt]) {
    for i in 1..pos.len() {
        let cp = &pos[i].clone();
        let xp = &pos[i - 1];
        let d = abs_d(cp, xp);
        if d == 2 {
            pos[i] = pt![pt_cmp!(xp.x, cp.x), pt_cmp!(xp.y, cp.y)];
        } else if d == 1 {
            if cp.x == xp.x {
                pos[i].y = pt_cmp!(xp.y, cp.y)
            } else {
                pos[i].x = pt_cmp!(xp.x, cp.x)
            }
        }
    }
}

pub fn part_2(input: &str) -> Option<u32> {
    let mut pp: &mut [Pt; 10] = &mut [pt![], pt![], pt![], pt![], pt![], pt![], pt![], pt![], pt![], pt![]];
    let mut paths: HashSet<Pt> = HashSet::new();
    for l in input.lines() {
        let (side, st) = parse_step(l);
        for _ in 0..st {
            match side {
                Side::X => pp[0].x += 1,
                Side::Y => pp[0].y += 1
            }
            adv(pp);
            paths.insert(pp[9].clone());
        }
    }
    Some(paths.len() as u32)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::assert_ex;

    #[test]
    fn test_part_1() {
        assert_ex!(part_1, 13);
    }

    #[test]
    fn test_part_2() {
        assert_ex!(part_2, 1);
    }
}
