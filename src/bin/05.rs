/*
 * Day 5: Supply Stacks
 * See [https://adventofcode.com/2022/day/5]
 */

use std::str::SplitAsciiWhitespace;

fn make_vecs(slen: usize) -> (usize, Box<[Vec<u8>]>) {
    let lx = (1 + slen) / 4;
    let v: Box<[Vec<u8>]> = vec![Vec::new(); lx].into_boxed_slice();
    (lx, v)
}

fn fill_line(ls: usize, bl: &[u8], v: &mut Box<[Vec<u8>]>) {
    let mut i = 1;
    let mut j = 0;
    loop {
        let c = bl[i];
        if c >= b'A' {
            v[j].push(c);
        }
        j += 1;
        i += 4;
        if j >= ls {
            break;
        }
    }
}

fn parse_opt_usize(it: &mut SplitAsciiWhitespace) -> usize {
    it.next();
    it.next().unwrap().parse().unwrap()
}

fn read_moves(s: &str) -> (usize, usize, usize) {
    let it = &mut s.split_ascii_whitespace();
    let c = parse_opt_usize(it);
    let f = parse_opt_usize(it) - 1;
    let t = parse_opt_usize(it) - 1;
    (c, f, t)
}

pub fn part_1(input: &str) -> Option<String> {
    let mut lines = input.lines().peekable();
    let (lx, mut v) = make_vecs(lines.peek().unwrap().len());
    let mut mode = 0;
    for l in lines {
        if l.is_empty() {
            mode = 1;
            v.iter_mut().for_each(|x| x.reverse());
            continue;
        }
        let bl = l.as_bytes();
        if mode == 0 {
            fill_line(lx, bl, &mut v);
        }
        if mode == 1 {
            let (c, f, t) = read_moves(l);

            for _ in 0..c {
                if let Some(u) = v[f].pop() {
                    v[t].push(u);
                }
            }
        }
    }
    let s: String = v.iter().map(|x| x.last().unwrap().clone() as char).collect();
    Some(s)
}

pub fn part_2(input: &str) -> Option<String> {
    let mut lines = input.lines().peekable();
    let mut mode = 0;
    let (lx, mut v) = make_vecs(lines.peek().unwrap().len());
    
    for l in lines {
        if l.is_empty() {
            mode = 1;
            v.iter_mut().for_each(|x| x.reverse());
            continue;
        }
        let bl = l.as_bytes();
        if mode == 0 {
            fill_line(lx, bl, &mut v);
        }
        if mode == 1 {
            let (c, f, t) = read_moves(l);

            let vf = &mut v[f];
            let vv = vf.split_off( vf.len() - c);
            v[t].extend_from_slice(vv.as_slice());
        }
    }
    let s: String = v.iter().map(|x| x.last().unwrap().clone() as char).collect();
    Some(s)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, "CMZ"); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, "MCD"); }
}