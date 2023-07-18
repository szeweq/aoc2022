/*
 * Day 11: Monkey in the Middle
 * See [https://adventofcode.com/2022/day/11]
 */

#[derive(Clone)]
struct Monkey {
    op: MonkeyOp,
    div: u64,
    throw_true: usize,
    throw_false: usize
}

#[derive(Clone)]
enum MonkeyOp {
    Add(u64), Mul(u64), Sqr
}

fn parse_u64(s: &str) -> u64 { s.parse().unwrap() }
fn parse_usize(s: &str) -> usize { s.parse().unwrap() }

fn parse_monkeys(input: &str) -> (Vec<Vec<u64>>, Vec<Monkey>) {
    let mut v = (Vec::new(), Vec::new());
    let mut nm = 0;
    let mut mi: Vec<u64> = vec![];
    let mut rm = Monkey {
        op: MonkeyOp::Sqr,
        div: 0,
        throw_true: 0,
        throw_false: 0
    };
    for l in input.lines() {
        if l.is_empty() {
            v.0.push(mi.to_vec());
            v.1.push(rm.clone());
            continue;
        }
        let bl = l.as_bytes();
        if bl[0] == b'M' {
            nm = parse_usize(&l[7..l.len()-1]);
        } else if bl[2] == b'S' {
            mi = l[18..].split(", ").map(parse_u64).collect();
        } else if bl[2] == b'O' {
            let ad = bl[23] == b'+';
            if &l[25..] == "old" {
                rm.op = if ad { MonkeyOp::Mul(2) } else { MonkeyOp::Sqr }
            } else {
                let n = l[25..].parse().unwrap();
                rm.op = if ad { MonkeyOp::Add(n) } else { MonkeyOp::Mul(n) }
            }
        } else if bl[2] == b'T' {
            rm.div = parse_u64(&l[21..]);
        } else if bl[4] == b'I' {
            if bl[7] == b't' {
                rm.throw_true = parse_usize(&l[29..]);
            } else if bl[7] == b'f' {
                rm.throw_false = parse_usize(&l[30..]);
            }
        }
    }
    if nm == v.1.len() {
        v.0.push(mi);
        v.1.push(rm);
    }
    v
}

fn op_item(item: u64, op: &MonkeyOp, lcd: u64) -> u64 {
    use MonkeyOp::*;
    let m = match *op {
        Add(x) => item + x,
        Mul(x) => item * x,
        Sqr => item * item
    };
    if lcd == 0 { m / 3 } else { m % lcd }
}

fn monkey_calc(vv: &mut [Vec<u64>], vm: &Vec<Monkey>, vi: &mut [usize], lcd: u64) {
    let vsz = vm.len();
    for i in 0..vsz {
        let m = &vm[i];
        let sz = vv[i].len();
        vi[i] += sz;
        for it in 0..sz {
            let oi = op_item(vv[i][it], &m.op, lcd);
            let thr = if oi % m.div == 0 { m.throw_true } else { m.throw_false };
            vv[thr].push(oi);
        }
        vv[i].clear();
    }
}

fn monkey_val(v: &mut [usize]) -> Option<usize> {
    v.sort();
    Some(v.iter().rev().take(2).product())
}

pub fn part_1(input: &str) -> Option<usize> {
    let (mut vv, vm) = parse_monkeys(input);
    let vsz = vm.len();
    let mut vi: Vec<usize> = vec![0; vsz];
    for _ in 0..20 {
        monkey_calc(&mut vv, &vm, &mut vi, 0);
    }
    monkey_val(&mut vi)
}

pub fn part_2(input: &str) -> Option<usize> {
    let (mut vv, vm) = parse_monkeys(input);
    let vsz = vm.len();
    let mut vi: Vec<usize> = vec![0; vsz];
    let lcd: u64 = vm.iter().map(|m| m.div).product();
    for _ in 0..10000 {
        monkey_calc(&mut vv, &vm, &mut vi, lcd);
    }
    monkey_val(&mut vi)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 10605); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 2713310158); }
}