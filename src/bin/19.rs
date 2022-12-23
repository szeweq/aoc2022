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

trait MineState {
    fn spend(&self, cost: u32) -> Self;
}

impl MineState for (u32, u32) {
    fn spend(&self, cost: u32) -> Self {
        (self.0, self.1 - cost)
    }
}

fn sim_blueprint(time: u32, costs: [u32; 6]) -> u32 {
    let [c_ore, c_clay, c_obs_o, c_obs_c, c_geo_o, c_geo_b] = costs;
    let state: [(u32, u32); 4] = [(1, 0), (0, 0), (0, 0), (0, 0)];
    let ototal = c_ore.max(c_clay).max(c_obs_o).max(c_geo_o);
    let mut vq = VecDeque::from([(state, time)]);
    let mut hs = HashSet::new();
    let mut max_geode = 0;
    let mut lt = time;
    let mut update = Vec::new();
    while let Some((cs, t)) = vq.pop_front() {
        let [
            mut s_ore,
            mut s_clay,
            mut s_obs,
            mut s_geo
        ] = cs;
        
        if t == 0 {
            if max_geode < s_geo.1 {
                max_geode = s_geo.1;
            }
            continue;
        }

        set_min!(s_ore.0, ototal);
        set_min!(s_clay.0, c_obs_c);
        set_min!(s_obs.0, c_geo_b);

        
        let nt = t - 1;
        let mine_ore = t * ototal - s_ore.0 * nt;
        set_min!(s_ore.1, mine_ore);
        let mine_clay = t * c_obs_c - s_clay.0 * nt;
        set_min!(s_clay.1, mine_clay);
        let mine_obs = t * c_geo_b - s_obs.0 * nt;
        set_min!(s_obs.1, mine_obs);

        if lt > t {
            hs.clear();
            lt = t;
        } else if lt == t && !hs.insert(([s_ore, s_clay, s_obs, s_geo], t)) {
            continue;
        }

        let lco = s_ore.1;
        let lcc = s_clay.1;
        let lcb = s_obs.1;

        s_ore.1 += s_ore.0;
        s_clay.1 += s_clay.0;
        s_obs.1 += s_obs.0;
        s_geo.1 += s_geo.0;

        update.push(([s_ore, s_clay, s_obs, s_geo], nt));
        if lco >= c_ore {
            let ns_ore = (s_ore.0 + 1, s_ore.1 - c_ore);
            update.push(([ns_ore, s_clay, s_obs, s_geo], nt));
        }
        if lco >= c_clay {
            let ns_ore = s_ore.spend(c_clay);
            let ns_clay = (s_clay.0 + 1, s_clay.1);
            update.push(([ns_ore, ns_clay, s_obs, s_geo], nt));
        }
        if lco >= c_obs_o && lcc >= c_obs_c {
            let ns_ore = s_ore.spend(c_obs_o);
            let ns_clay = s_clay.spend(c_obs_c);
            let ns_obs = (s_obs.0 + 1, s_obs.1);
            update.push(([ns_ore, ns_clay, ns_obs, s_geo], nt));
        }
        if lco >= c_geo_o && lcb >= c_geo_b {
            let ns_ore = s_ore.spend(c_geo_o);
            let ns_obs = s_obs.spend(c_geo_b);
            let ns_geo = (s_geo.0 + 1, s_geo.1);
            update.push(([ns_ore, s_clay, ns_obs, ns_geo], nt));
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