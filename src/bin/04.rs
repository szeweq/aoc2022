use aoc2022::util;

#[derive(Debug, Clone, Copy)]
struct SRange {
    l: u32,
    h: u32
}

impl SRange {
    fn within(&self, other: &SRange) -> bool {
        let d = self.h - self.l;
        let od = other.h - other.l;
        if od >= d {
            self.l >= other.l && self.h <= other.h
        } else {
            other.l >= self.l && other.h <= self.h
        }
    }

    fn overlap(&self, other: &SRange) -> bool {
        let r = self.l..=self.h;
        self.within(other) || r.contains(&other.l) || r.contains(&other.h)
    }
}

fn pair_range_str(s: &str) -> (SRange, SRange) {
    let (p1, p2) = util::split_str_on(s, ',');
    (range_str(p1), range_str(p2))
}

fn range_str(s: &str) -> SRange {
    let (a, b) = util::split_str_on(s, '-');
    SRange {
        l: a.parse().unwrap(),
        h: b.parse().unwrap()
    }
}

pub fn part_2(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for l in lines {
        let (r1, r2) = pair_range_str(l);
        if r1.overlap(&r2) {
            sum += 1;
        }
    }
    Some(sum)
}

pub fn part_1(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for l in lines {
        let (r1, r2) = pair_range_str(l);
        if r1.within(&r2) {
            sum += 1;
        }
    }
    Some(sum)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() {
        assert_ex!(part_1, 2);
    }

    #[test]
    fn test_part_2() {
        assert_ex!(part_2, 4);
    }
}