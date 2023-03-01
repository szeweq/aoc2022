/*
 * Day 21: Monkey Math
 * See [https://adventofcode.com/2022/day/21]
 */

use std::{collections::{HashMap, HashSet}};

type Mn = u32;

#[derive(Debug, Clone, Copy)]
enum OpType {
    N(i64), O(Mn, Op, Mn)
}

macro_rules! is_op {
    ($v:expr) => {
        if let OpType::O(_, _, _) = $v { true } else { false }
    };
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add, Sub, Mul, Div
}

impl Op {
    fn do_op(&self, a: i64, b: i64) -> i64 {
        use Op::*;
        match self {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b
        }
    }
}

macro_rules! mn {
    [$f:expr] => {
        u32::from_ne_bytes([$f[0], $f[1], $f[2], $f[3]])
    };
    [$f:expr;$s:expr] => {
        u32::from_ne_bytes([$f[$s], $f[$s+1], $f[$s+2], $f[$s+3]])
    };
}

const M_ROOT: Mn = u32::from_ne_bytes(*b"root");
const M_HUMN: Mn = u32::from_ne_bytes(*b"humn");

fn parse_line(l: &str) -> (Mn, OpType) {
    let bl = l.as_bytes();
    let r: Mn = mn![bl];
    let id = bl[6].is_ascii_digit();
    let op = if id { OpType::N(l[6..].parse().unwrap()) } else {
        let a = mn![bl; 6];
        let b = mn![bl; 13];
        let o = match bl[11] {
            b'+' => Op::Add,
            b'-' => Op::Sub,
            b'*' => Op::Mul,
            b'/' => Op::Div,
            _ => { panic!("Not allowed character") }
        };
        OpType::O(a, o, b)
    };
    (r, op)
}

fn calculate(map: &mut HashMap<Mn, OpType>, queue: &mut Vec<Mn>, start: Mn) -> Option<i64> {
    use OpType::*;
    queue.push(start);
    while let Some(cm) = queue.last() {
        if let O(a, op, b) = &map[cm] {
            let av = map[a];
            let bv = map[b];
            if let (N(na), N(nb)) = (av, bv) {
                let nn = op.do_op(na, nb);
                map.insert(*cm, N(nn));
                queue.pop();
                continue;
            }
            if is_op!(av) {
                queue.push(*a);
            }
            if is_op!(bv) {
                queue.push(*b);
            }
        }
    }
    if let OpType::N(x) = map[&start] { Some(x) } else { None }
}

pub fn part_1(input: &str) -> Option<i64> {
    let mut m = input.lines().map(parse_line).collect::<HashMap<_, _>>();
    let mut q: Vec<Mn> = Vec::new();
    calculate(&mut m, &mut q, M_ROOT)
}

/// This optimization algorithm assumes that each monkey yells to another one.
fn optimize(map: &mut HashMap<Mn, OpType>, queue: &mut Vec<Mn>, start: Mn) -> Option<Mn> {
    use OpType::*;
    let mut hs = HashSet::new();
    let mut hv = None;
    queue.push(start);
    while let Some(cm) = queue.pop() {
        let cv = map[&cm];
        if let O(a, op, b) = cv {
            let av = map[&a];
            let bv = map[&b];
            if a == M_HUMN || b == M_HUMN {
                hv = Some(queue[1])
            }
            if a != M_HUMN && b != M_HUMN {
                if let (N(na), N(nb)) = (&av, &bv) {
                    if matches!(op, Op::Mul) && (na > &20 || nb > &20) {
                        continue;
                    }
                    map.remove(&a);
                    map.remove(&b);
                    let nn = op.do_op(*na, *nb);
                    map.insert(cm, N(nn));
                    continue;
                }
            }
            if hs.insert(cm) {
                queue.push(cm);
                if is_op!(av) {
                    queue.push(a);
                }
                if is_op!(bv) {
                    queue.push(b);
                }
            }
        }
    }
    map.shrink_to_fit();
    hv
}

fn resolve(map: &mut HashMap<Mn, OpType>, queue: &mut Vec<Mn>, start: Mn, rv: i64) -> Option<i64> {
    use OpType::*;
    queue.push(start);
    let mut opv = vec![];
    while let Some(cm) = queue.pop() {
        let cv = map[&cm];
        if let O(a, op, b) = cv {
            let av = map[&a];
            let bv = map[&b];
            if a == M_HUMN || b == M_HUMN {
                let N(on) = (if a == M_HUMN { bv } else { av }) else { panic!("Not optimized!") };
                opv.push((op, on));
            } else if opv.len() > 0 {
                let on = match (av, bv) {
                    (N(_), N(_)) | (O(_, _, _), O(_, _, _)) => panic!("Not optimized!"),
                    (N(x), O(_, _, _)) => x,
                    (O(_, _, _), N(x)) => x
                };
                opv.push((op, on));
            } else {
                queue.push(cm);
                if is_op!(av) {
                    queue.push(a);
                }
                if is_op!(bv) {
                    queue.push(b);
                }
            }
        }
    }
    let mut uhs = HashSet::new();
    uhs.insert(rv);
    while let Some((o, n)) = opv.pop() {
        let cuv = uhs.drain().collect::<Vec<_>>();
        for v in cuv {
            match o {
                Op::Div => {
                    let mv = v * n;
                    uhs.extend(mv..mv+n);
                }
                Op::Add => { uhs.insert(v - n); }
                Op::Sub => { uhs.insert(v + n); }
                Op::Mul => { uhs.insert(v / n); }
            }
        }
    }
    uhs.into_iter().min()
    
}

pub fn part_2(input: &str) -> Option<i64> {
    use OpType::*;

    let mut m = input.lines().map(parse_line).collect::<HashMap<_, _>>();
    let mut q: Vec<Mn> = Vec::new();
    let O(ma, _, mb) = m[&M_ROOT] else { return None; };
    let ccom = optimize(&mut m, &mut q, M_ROOT).unwrap();
    let (calc, comp) = if ccom == ma { (mb, ma) } else { (ma, mb) };
    let calcv = calculate(&mut m, &mut q, calc).unwrap();
    if cfg!(test) {
        resolve(&mut m.clone(), &mut q, comp, calcv)
    } else {
        let mut hn = 0;
        let mut dt = 1 << 26;
        loop {
            *m.get_mut(&M_HUMN).unwrap() = N(hn);
            let mut cm = m.clone();
            let x = calculate(&mut cm, &mut q, comp).unwrap();
            if x == calcv {
                loop {
                    *m.get_mut(&M_HUMN).unwrap() = N(hn-1);
                    let mut cm = m.clone();
                    let x = calculate(&mut cm, &mut q, comp).unwrap();
                    if x != calcv {
                        break;
                    }
                    hn -= 1;
                }
                return Some(hn);
            } else if x < calcv {
                hn -= (dt << 1)-1;
                dt >>= 1;
            } else {
                hn += dt;
            }
        }
    }
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 152); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 301); }
}