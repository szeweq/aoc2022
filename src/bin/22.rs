use std::{collections::{HashMap, BTreeSet}};

const SEC_SIZE: isize = if cfg!(test) { 4 } else { 50 };

type Pt = (isize, isize);

trait PosMath {
    fn p_add(&self, other: Self) -> Self;
    fn p_sub(&self, other: Self) -> Self;
    fn p_mul(&self, by: isize) -> Self;
    fn p_div(&self, by: isize) -> Self;
}

impl PosMath for Pt {
    fn p_add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
    fn p_sub(&self, other: Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
    fn p_mul(&self, by: isize) -> Self {
        (self.0 * by, self.1 * by)
    }
    fn p_div(&self, by: isize) -> Self {
        (self.0 / by, self.1 / by)
    }
}

fn pt_fix(pt: Pt, from: Dir, to: Dir) -> Pt {
    use Dir::*;
    const SEC_LAST: isize = SEC_SIZE - 1;
    match (from, to) {
        (R, R) => (pt.0, 0),
        (R, D) => (0, SEC_LAST-pt.0),
        (R, L) => (SEC_LAST-pt.0, SEC_LAST),
        (R, U) => (SEC_LAST, pt.0),
        (D, R) => (SEC_LAST-pt.1, 0),
        (D, D) => (0, pt.1),
        (D, L) => (pt.1, SEC_LAST),
        (D, U) => (SEC_LAST, SEC_LAST-pt.1),
        (L, R) => (SEC_LAST-pt.0, 0),
        (L, D) => (0, pt.0),
        (L, L) => (pt.0, SEC_LAST),
        (L, U) => (SEC_LAST, SEC_LAST-pt.0),
        (U, R) => (pt.1, 0),
        (U, D) => (0, SEC_LAST-pt.1),
        (U, L) => (SEC_LAST-pt.1, SEC_LAST),
        (U, U) => (SEC_LAST, pt.1),
    }
}

fn parse(mut input: &str) -> (HashMap<Pt, bool>, Vec<(u32, usize)>) {
    if input.ends_with('\n') {
        input = &input[..input.len()-1];
    }
    let ix = input.rfind('\n').unwrap();
    let hm = input[..ix].lines()
        .enumerate()
        .flat_map(|(i, l)| {
            let ni = i;
            l.bytes()
                .enumerate()
                .filter_map(move |(j, b)| {
                    if b == b' ' { None } else { Some(((ni as isize, j as isize), b == b'#')) }
                })
        }).collect::<HashMap<_,_>>();
    
    let instr = &input[ix+1..];
    let mut n = 0;
    let mut movs = vec![];
    let instr_b = instr.as_bytes();
    for i in 1..instr.len() {
        let b = instr_b[i];
        if b == b'L' || b == b'R' {
            let num: u32 = instr[n..i].parse().unwrap();
            movs.push((num, if b == b'L' { 3 } else { 1 }));
            n = i + 1;
        }
    }
    let lnum = instr[n..].parse().unwrap();
    movs.push((lnum, 0));
    (hm, movs)
}

const RP: [Pt; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    R, D, L, U
}

impl From<usize> for Dir {
    fn from(value: usize) -> Self {
        use Dir::*;
        match value % 4 {
            0 => R,
            1 => D,
            2 => L,
            3 => U,
            _ => panic!("Unknown value")
        }
    }
}

pub fn part_1(input: &str) -> Option<isize> {
    let (hm, movs) = parse(input);
    let mut pos = hm.keys().min().unwrap().clone();
    let mut cf = 0;
    let mut face = RP[cf];
    for i in 0..movs.len() {
        let (mn, r) = movs[i];
        for _ in 0..mn {
            let nextpos = pos.p_add(face);
            match hm.get(&nextpos) {
                Some(true) => {
                    break;
                }
                Some(false) => {
                    pos = nextpos;
                }
                None => {
                    let stepback = face.p_mul(SEC_SIZE);
                    let mut wpos = nextpos.p_sub(stepback);
                    loop {
                        let nwpos = wpos.p_sub(stepback);
                        if !hm.contains_key(&nwpos) {
                            break;
                        }
                        wpos = nwpos;
                    }
                    match hm.get(&wpos) {
                        None | Some(true) => {
                            break;
                        }
                        Some(false) => {
                            pos = wpos;
                        }
                    }
                }
            }
        }
        if r != 0 {
            cf = (cf + r) % 4;
            face = RP[cf];
        }
    }
    Some((pos.0+1)*1000 + (pos.1+1)*4 + cf as isize)
}

fn compute_sectors(hm: &HashMap<Pt, bool>) -> (Vec<Pt>, HashMap<Pt, usize>) {
    let mut bts = BTreeSet::new();
    for k in hm.keys().copied() {
        let ss = (k.0 / SEC_SIZE, k.1 / SEC_SIZE);
        bts.insert(ss);
    }
    let h1 = bts.iter().copied().collect();
    let h2 = bts.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    (h1, h2)
}

fn compute_folds(_: &HashMap<Pt, usize>) -> HashMap<(usize, Dir), (usize, Dir)> {
    use Dir::*;
    // HARDCODED VALUES
    let ar: [((usize, Dir), (usize, Dir)); 14] = if cfg!(test) {
        [
            ((0, R), (5, L)), ((0, L), (2, D)), ((0, U), (1, D)),
            ((1, D), (4, U)), ((1, L), (5, U)), ((1, U), (0, D)),
            ((2, D), (4, R)), ((2, U), (0, R)),
            ((3, R), (5, D)),
            ((4, D), (1, U)), ((4, L), (2, U)),
            ((5, R), (0, L)), ((5, D), (1, R)), ((5, U), (3, L))
        ]
    } else {
        // Try to determine values for your own input
        // Mine looks like this:
        //     [0][1]
        //     [2]
        //  [3][4]
        //  [5]
        [
            ((0, L), (3, R)), ((0, U), (5, R)),
            ((1, R), (4, L)), ((1, D), (2, L)), ((1, U), (5, U)),
            ((2, R), (1, U)), ((2, L), (3, D)),
            ((3, L), (0, R)), ((3, U), (2, R)),
            ((4, R), (1, L)), ((4, D), (5, L)),
            ((5, R), (4, U)), ((5, D), (1, D)), ((5, L), (0, D))
        ]
    };
    ar.into_iter().collect()
}

pub fn part_2(input: &str) -> Option<isize> {
    let (hm, movs) = parse(input);
    let (si_to_sp, sp_to_si) = compute_sectors(&hm);
    let folds = compute_folds(&sp_to_si);
    
    let mut pos = hm.keys().min().unwrap().clone();
    let mut cf = Dir::R;
    let mut face = RP[cf as usize];
    for i in 0..movs.len() {
        //println!("At {:?} moving {:?}", pos, cf);
        let (mn, r) = movs[i];
        for _ in 0..mn {
            let nextpos = (pos.0 + face.0, pos.1 + face.1);
            match hm.get(&nextpos) {
                Some(true) => {
                    break;
                }
                Some(false) => {
                    pos = nextpos;
                }
                None => {
                    let mut sec = pos.p_div(SEC_SIZE);
                    let cs = sp_to_si[&sec];
                    sec = sec.p_mul(SEC_SIZE);
                    let pos_in_sec = pos.p_sub(sec);
                    let wf = folds[&(cs, cf)];
                    let nspos = si_to_sp[wf.0].p_mul(SEC_SIZE);
                    let wpos = pt_fix(pos_in_sec, cf, wf.1).p_add(nspos);
                    match hm.get(&wpos) {
                        None | Some(true) => {
                            break;
                        }
                        Some(false) => {
                            pos = wpos;
                            cf = wf.1;
                            face = RP[cf as usize];
                        }
                    }
                }
            }
        }
        if r != 0 {
            let ncf = (cf as usize + r) % 4;
            face = RP[ncf];
            cf = ncf.into();
        }
    }
    Some((pos.0+1)*1000 + (pos.1+1)*4 + cf as isize)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 6032); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 5031); }
}