/*
 * Day 19: Not Enough Minerals
 * See [https://adventofcode.com/2022/day/19]
 */
use std::collections::{VecDeque, HashSet};

fn parse_line(line: &str) -> [u32; 6] {
    let mut sp = line.split(" Each ");
    sp.next().unwrap();
    let oc = (sp.next().unwrap().as_bytes()[16] - b'0') as u32;
    let cc = (sp.next().unwrap().as_bytes()[17] - b'0') as u32;
    let obs = sp.next().unwrap();
    let boc = (obs.as_bytes()[21] - b'0') as u32;
    let bcc = obs[31..].split_once(' ').unwrap().0.parse().unwrap();
    let geode = sp.next().unwrap();
    let goc = (geode.as_bytes()[18] - b'0') as u32;
    let gbc = geode[28..].split_once(' ').unwrap().0.parse().unwrap();
    [oc, cc, boc, bcc, goc, gbc]
}

macro_rules! set_min {
    ($e:expr,$v:expr) => {
        if $e > $v {
            $e = $v;
        }
    };
}

const M_ORE: usize = 0;
const M_CLAY: usize = 1;
const M_OBS: usize = 2;
const M_GEODE: usize = 3;

fn sim_blueprint(time: u32, costs: [u32; 6]) -> u32 {
    let [c_ore, c_clay, c_obs_o, c_obs_c, c_geo_o, c_geo_b] = costs;
    let state: [[u32; 4]; 2] = [[1, 0, 0, 0], [0, 0, 0, 0]];
    let ototal = c_ore.max(c_clay).max(c_obs_o).max(c_geo_o);
    let mut vq = VecDeque::from([(state, time)]);
    let mut hs = HashSet::new();
    let mut max_geode = 0;
    let mut lt = time;
    let mut update = Vec::new();
    while let Some((cs, t)) = vq.pop_front() {
        let [mut miners, mut mats] = cs;
        
        if t == 0 {
            if max_geode < mats[M_GEODE] {
                max_geode = mats[M_GEODE];
            }
            continue;
        }

        set_min!(miners[M_ORE], ototal);
        set_min!(miners[M_CLAY], c_obs_c);
        set_min!(miners[M_OBS], c_geo_b);

        
        let nt = t - 1;
        let mine_ore = t * ototal - miners[M_ORE] * nt;
        set_min!(mats[M_ORE], mine_ore);
        let mine_clay = t * c_obs_c - miners[M_CLAY] * nt;
        set_min!(mats[M_CLAY], mine_clay);
        let mine_obs = t * c_geo_b - miners[M_OBS] * nt;
        set_min!(mats[M_OBS], mine_obs);

        if lt > t {
            hs.clear();
            lt = t;
        } else if lt == t && !hs.insert([miners, mats]) {
            continue;
        }

        let [lco, lcc, lcb, _] = mats;

        mats[M_ORE] += miners[M_ORE];
        mats[M_CLAY] += miners[M_CLAY];
        mats[M_OBS] += miners[M_OBS];
        mats[M_GEODE] += miners[M_GEODE];

        update.push(([miners, mats], nt));
        if lco >= c_ore {
            let mut nminers = miners.clone();
            let mut nmats = mats.clone();
            nminers[M_ORE] += 1;
            nmats[M_ORE] -= c_ore;
            update.push(([nminers, nmats], nt));
        }
        if lco >= c_clay {
            let mut nminers = miners.clone();
            let mut nmats = mats.clone();
            nminers[M_CLAY] += 1;
            nmats[M_ORE] -= c_clay;
            update.push(([nminers, nmats], nt));
        }
        if lco >= c_obs_o && lcc >= c_obs_c {
            let mut nminers = miners.clone();
            let mut nmats = mats.clone();
            nminers[M_OBS] += 1;
            nmats[M_ORE] -= c_obs_o;
            nmats[M_CLAY] -= c_obs_c;
            update.push(([nminers, nmats], nt));
        }
        if lco >= c_geo_o && lcb >= c_geo_b {
            let mut nminers = miners.clone();
            let mut nmats = mats.clone();
            nminers[M_GEODE] += 1;
            nmats[M_ORE] -= c_geo_o;
            nmats[M_OBS] -= c_geo_b;
            update.push(([nminers, nmats], nt));
        }
        vq.extend(&update);
        update.clear();
    }
    max_geode
}

pub fn part_1(input: &str) -> Option<u32> {
    let bpv = input.lines().map(parse_line).enumerate().collect::<Vec<_>>();
    Some(bpv.into_iter()
        .map(|(i, c)| (i as u32 + 1) * sim_blueprint(24, c))
        .sum()
    )
}

pub fn part_2(input: &str) -> Option<u32> {
    let bpv = input.lines().take(3).map(parse_line).collect::<Vec<_>>();
    Some(bpv.into_iter()
        .map(|c| sim_blueprint(32, c))
        .product()
    )
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 33); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 3472); }
}