/*
 * Day 21: Monkey Math
 * See [https://adventofcode.com/2022/day/21]
 */

use std::{collections::{HashMap, HashSet}};

type Mn = [u8; 4];

#[derive(Debug, Clone)]
enum OpType {
    Num(i64), Op(Mn, Op, Mn)
}

macro_rules! is_op {
    ($v:expr) => {
        if let OpType::Op(_, _, _) = $v { true } else { false }
    };
}

#[derive(Debug, Clone)]
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
    [$f:expr;$s:expr] => {
        [$f[$s], $f[$s+1], $f[$s+2], $f[$s+3]]
    };
}

const M_ROOT: Mn = *b"root";
const M_HUMN: Mn = *b"humn";

fn parse_line(l: &str) -> (Mn, OpType) {
    let bl = l.as_bytes();
    let r: Mn = mn![bl; 0];
    let id = bl[6].is_ascii_digit();
    let op = if id { OpType::Num(l[6..].parse().unwrap()) } else {
        let a = mn![bl; 6];
        let b = mn![bl; 13];
        let o = match l.as_bytes()[11] {
            b'+' => Op::Add,
            b'-' => Op::Sub,
            b'*' => Op::Mul,
            b'/' => Op::Div,
            _ => { panic!("Not allowed character") }
        };
        OpType::Op(a, o, b)
    };
    (r, op)
}

fn calculate(map: &mut HashMap<Mn, OpType>, queue: &mut Vec<Mn>, start: Mn) -> Option<i64> {
    use OpType::*;
    queue.push(start);
    while let Some(cm) = queue.pop() {
        let cv = &map[&cm];
        if let Op(a, op, b) = cv {
            let av = &map[a];
            let bv = &map[b];
            if let (Num(na), Num(nb)) = (av, bv) {
                let nn = op.do_op(*na, *nb);
                map.insert(cm, Num(nn));
                continue;
            }
            queue.push(cm);
            if is_op!(av) {
                queue.push(*a);
            }
            if is_op!(bv) {
                queue.push(*b);
            }
        }
    }
    if let OpType::Num(x) = map.get(&start).unwrap() { Some(*x) } else { None }
}

pub fn part_1(input: &str) -> Option<i64> {
    let mut m = input.lines().map(parse_line).collect::<HashMap<_, _>>();
    let mut q: Vec<Mn> = Vec::new();
    calculate(&mut m, &mut q, M_ROOT)
}

/// This optimization algorithm assumes that each monkey yells to another one.
fn optimize(map: &mut HashMap<Mn, OpType>, queue: &mut Vec<Mn>, start: Mn) {
    use OpType::*;
    let mut hs = HashSet::new();
    queue.push(start);
    while let Some(cm) = queue.pop() {
        let cv = map.get(&cm).unwrap().clone();
        if let Op(a, op, b) = cv {
            let av = map.get(&a).unwrap().clone();
            let bv = map.get(&b).unwrap().clone();
            if a != M_HUMN && b != M_HUMN {
                if let (Num(na), Num(nb)) = (&av, &bv) {
                    map.remove(&a);
                    map.remove(&b);
                    let nn = op.do_op(*na, *nb);
                    map.insert(cm, Num(nn));
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
}

pub fn part_2(input: &str) -> Option<i64> {
    use OpType::*;
    use std::cmp::Ordering::*;

    let mut m = input.lines().map(parse_line).collect::<HashMap<_, _>>();
    let mut q: Vec<Mn> = Vec::new();
    let Op(ma, _, mb) = m.get(&M_ROOT).unwrap().clone() else { return None; };
    optimize(&mut m, &mut q, M_ROOT);
    let (calc, comp) = match (&m[&ma], &m[&mb]) {
        (Num(_), Op(_, _, _)) => (ma, mb),
        (Op(_, _, _), Num(_)) => (mb, ma),
        (Op(_, _, _), Op(_, _, _)) | (Num(_), Num(_)) => panic!("This cannot be solved here!"),
    };
    let OpType::Num(calcv) = m[&calc] else { return None; };
    let mut hv = 0;
    let mut d = 1 << 48; // Arbitrary number
    let mut check = false;
    loop {
        let mut cm = m.clone();
        *cm.get_mut(&M_HUMN).unwrap() = Num(hv);

        let x = calculate(&mut cm, &mut q, comp)?;
        if check {
            if x != calcv {
                return Some(hv+1);
            }
            hv -= 1;
        } else {
            match x.cmp(&calcv) {
                Less => {
                    d /= 2;
                    hv -= d;
                },
                Equal => {
                    check = true
                },
                Greater => hv += d,
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