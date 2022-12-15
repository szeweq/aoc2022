
pub fn part_1(input: &str) -> Option<u32> {
    let mut maxcal: u32 = 0;

    let mut cal: u32 = 0;
    for l in input.lines() {
        if l.is_empty() {
            if maxcal < cal {
                maxcal = cal;
            }
            cal = 0;
        } else {
            let cc: u32 = l.parse().unwrap();
            cal += cc;
        }
    }
    if maxcal < cal {
        maxcal = cal;
    }
    
    Some(maxcal)
}

pub fn part_2(input: &str) -> Option<u32> {
    let mut elfc: Vec<u32> = Vec::new();
    let mut cal: u32 = 0;
    for l in input.lines() {
        if l.is_empty() {
            elfc.push(cal);
            cal = 0;
        } else {
            let cc: u32 = l.parse().unwrap();
            cal += cc;
        }
    }
    if cal != 0 {
        elfc.push(cal);
    }

    elfc.sort_by(|a, b| b.cmp(a));
    Some(elfc.iter().take(3).sum())
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() {
        assert_ex!(part_1, 24000);
    }

    #[test]
    fn test_part_2() {
        assert_ex!(part_2, 45000);
    }
}