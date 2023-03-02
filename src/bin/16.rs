/*
 * Day 16: Proboscidea Volcanium
 * See [https://adventofcode.com/2022/day/16]
 */
use std::collections::HashMap;

fn parse_vals(input: &str) -> (HashMap<&str, (usize, Vec<&str>)>, Vec<&str>) {
    let mut m = HashMap::new();
    let mut ks = Vec::new();
    for l in input.lines() {
        let (a, b) = l.split_once(';').unwrap();
        let src = &a[6..8];
        let rate = (&a[23..]).parse::<usize>().unwrap();
        let dst = (if b.as_bytes()[7] == b's' { &b[24..] } else { &b[23..] }).split(", ").collect::<Vec<&str>>();
        m.insert(src, (rate, dst));
        ks.push(src);
    }
    (m, ks)
}

fn parse_vals2(input: &str, start_at: &str) -> (Vec<(usize, Vec<usize>)>, usize) {
    let (vals, poss) = parse_vals(input);
    let v = poss.iter()
        .map(|k| {
            let p = &vals[*k];
            let u = p.0;
            let l = p.1.iter()
                .map(|to| poss.iter().position(|z| *z == *to).unwrap())
                .collect::<Vec<_>>();
            (u, l)
        })
        .collect();
    (v, poss.iter().position(|z| *z == start_at).unwrap())
}

pub fn part_1(input: &str) -> Option<usize> {
    let (vals, start_at) = parse_vals2(input, "AA");
    let mut current = &mut Vec::new();
    let mut newer = &mut Vec::new();

    current.push((start_at, 0, 0));
    let mut pr_state: HashMap<(_, _), usize> = HashMap::new();
    const TIME: usize = 30;
    for t in 1..=TIME {
        newer.clear();
        for i in 0..current.len() {
            let (ix, o, pr) = current[i];
            let ki = 1 << ix;
            let k = (ki, o);
            if let Some(&x) = pr_state.get(&k) {
                if pr <= x {
                    continue;
                }
            }
            pr_state.insert(k, pr);

            let vtpl = &vals[ix];
            let ip = vtpl.0;
            if ki & o == 0 && ip > 0 {
                newer.push((ix, o | ki, pr + ip * (TIME - t)));
            }
            for d in &vtpl.1 {
                newer.push((*d, o, pr));
            }
        }
        (current, newer) = (&mut *newer, &mut *current);
    }
    current.into_iter().map(|(_, _, p)| *p).max()
}

struct Part2Solver {
    valves: Vec<(usize, Vec<usize>)>,
    cache: HashMap<(u32, u64, usize), usize>
}

impl Part2Solver {
    fn get_outs(&self, loc: usize) -> &Vec<usize> {
        &self.valves.get(loc).unwrap().1
    }

    fn solve(&mut self, tim: u32, cs: usize, step: (usize, usize, usize, u64)) -> Option<usize> {
        if tim > 26 {
            return Some(cs);
        }
    
        let (mloc, eloc, fr, open) = step;
        let mkf = 1 << mloc;
        let ekf = 1 << eloc;

        let ck = (tim, mkf | ekf, fr);
        if let Some(cached_value) = self.cache.get(&ck) {
            if *cached_value >= cs {
                return None;
            }
        }
        self.cache.insert(ck, cs);

        let mfr = self.valves[mloc].0;
        let efr = self.valves[eloc].0;
    
        let m_canopen = mfr > 0 && (open & mkf == 0);
        let e_canopen = efr > 0 && (open & ekf == 0);
        let mut opts = Vec::new();
    
        if m_canopen {
            let n_open = open | mkf;
    
            for n_eloc in self.get_outs(eloc) {
                opts.push((mloc, *n_eloc, fr + mfr, n_open));
            }
        }
    
        if e_canopen {
            let n_open = open | ekf;
    
            for n_mloc in self.get_outs(mloc) {
                opts.push((*n_mloc, eloc, fr + efr, n_open));
            }
        }
    
        if e_canopen && m_canopen && mloc != eloc {
            let n_open = open | mkf | ekf;
            opts.push((mloc, eloc, fr + mfr + efr, n_open));
        }
    
        for n_eloc in self.get_outs(eloc) {
            for n_mloc in self.get_outs(mloc) {
                opts.push((*n_mloc, *n_eloc, fr, open));
            }
        }

        let mut rmax = 0;
        for n in opts {
            if let Some(r) = self.solve(tim + 1, cs + fr, n) {
                if rmax < r {
                    rmax = r;
                }
            }
        }

        Some(rmax)
    }
}

pub fn part_2(input: &str) -> Option<usize> {
    let (vals, start_at) = parse_vals2(input, "AA");
    let mut p2s = Part2Solver {
        valves: vals,
        cache: HashMap::new()
    };

    p2s.solve(1, 0, (start_at, start_at, 0, 0))
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 1651); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 1707); }
}