/*
 * Day 19: Not Enough Minerals
 * See [https://adventofcode.com/2022/day/19]
 */
use std::collections::{VecDeque, HashSet};

fn parse_line(line: &str) -> [u16; 6] {
    let mut sp = line.split(" Each ");
    sp.next().unwrap();
    let oc = (sp.next().unwrap().as_bytes()[16] - b'0') as u16;
    let cc = (sp.next().unwrap().as_bytes()[17] - b'0') as u16;
    let obs = sp.next().unwrap();
    let boc = (obs.as_bytes()[21] - b'0') as u16;
    let bcc = obs[31..].split_once(' ').unwrap().0.parse().unwrap();
    let geode = sp.next().unwrap();
    let goc = (geode.as_bytes()[18] - b'0') as u16;
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct MineState { // m_ = miners, r_ = resources
    m_ore: u16,
    m_clay: u16,
    m_obs: u16,
    m_geode: u16,
    r_ore: u16,
    r_clay: u16,
    r_obs: u16,
    r_geode: u16
}

fn sim_blueprint(time: u16, costs: [u16; 6]) -> u16 {
    let [c_ore, c_clay, c_obs_o, c_obs_c, c_geo_o, c_geo_b] = costs;
    let state = MineState {m_ore: 1, m_clay: 0, m_obs: 0, m_geode: 0, r_ore: 0, r_clay: 0, r_obs: 0, r_geode: 0};
    let ototal = c_ore.max(c_clay).max(c_obs_o).max(c_geo_o);
    let mut vq = VecDeque::from([(state, time)]);
    let mut hs = HashSet::new();
    let mut max_geode = 0;
    let mut lt = time;
    while let Some((mut cs, t)) = vq.pop_front() {
        if t == 0 {
            if max_geode < cs.r_geode {
                max_geode = cs.r_geode;
            }
            continue;
        }

        set_min!(cs.m_ore, ototal);
        set_min!(cs.m_clay, c_obs_c);
        set_min!(cs.m_obs, c_geo_b);

        
        let nt = t - 1;
        let mine_ore = t * ototal - cs.m_ore * nt;
        set_min!(cs.r_ore, mine_ore);
        let mine_clay = t * c_obs_c - cs.m_clay * nt;
        set_min!(cs.r_clay, mine_clay);
        let mine_obs = t * c_geo_b - cs.m_obs * nt;
        set_min!(cs.r_obs, mine_obs);

        if lt > t {
            hs.clear();
            lt = t;
        } else if lt == t && !hs.insert(cs.clone()) {
            continue;
        }

        let mut ncs = cs.clone();
        ncs.r_ore += ncs.m_ore;
        ncs.r_clay += ncs.m_clay;
        ncs.r_obs += ncs.m_obs;
        ncs.r_geode += ncs.m_geode;

        vq.push_back((ncs.clone(), nt));
        if cs.r_ore >= c_ore {
            let mut vcs = ncs.clone();
            vcs.m_ore += 1;
            vcs.r_ore -= c_ore;
            vq.push_back((vcs, nt));
        }
        if cs.r_ore >= c_clay {
            let mut vcs = ncs.clone();
            vcs.m_clay += 1;
            vcs.r_ore -= c_clay;
            vq.push_back((vcs, nt));
        }
        if cs.r_ore >= c_obs_o && cs.r_clay >= c_obs_c {
            let mut vcs = ncs.clone();
            vcs.m_obs += 1;
            vcs.r_ore -= c_obs_o;
            vcs.r_clay -= c_obs_c;
            vq.push_back((vcs, nt));
        }
        if cs.r_ore >= c_geo_o && cs.r_obs >= c_geo_b {
            let mut vcs = ncs.clone();
            vcs.m_geode += 1;
            vcs.r_ore -= c_geo_o;
            vcs.r_obs -= c_geo_b;
            vq.push_back((vcs, nt));
        }
    }
    max_geode
}

pub fn part_1(input: &str) -> Option<u16> {
    let bpv = input.lines().map(parse_line).enumerate().collect::<Vec<_>>();
    Some(bpv.into_iter()
        .map(|(i, c)| (i as u16 + 1) * sim_blueprint(24, c))
        .sum()
    )
}

pub fn part_2(input: &str) -> Option<u16> {
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