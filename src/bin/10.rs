/*
 * Day 10: Cathode-Ray Tube
 * See [https://adventofcode.com/2022/day/10]
 */

pub fn part_1(input: &str) -> Option<i32> {
    let mut cycles = 1;
    let mut x = 1;
    let mut rc = 20;
    let mut sum = 0;
    for l in input.lines() {
        if l == "noop" {
            cycles += 1;
        } else if let Ok(v) = l[5..].parse::<i32>() {
            if cycles + 1 == rc {
                sum += rc * x;
                rc += 40;
            }
            cycles += 2;
            x += v;
        }
        if cycles == rc {
            sum += rc * x;
            rc += 40;
        }
    }
    Some(sum)
}

pub fn part_2(input: &str) -> Option<String> {
    let mut cycles = 0;
    let mut x = 1i32;
    let vv = &mut ['░'; 40*6];

    macro_rules! cdraw {
        () => {
            let ch = cycles % 40;
            let z = (ch as i32) - x;
            if (-1..2i32).contains(&z) {
                vv[cycles] = '█';
            }
            cycles += 1;
        };
    }

    for l in input.lines() {
        if l == "noop" {
                cdraw!();
        } else if let Ok(v) = l[5..].parse::<i32>() {
                cdraw!();
                cdraw!();
                x += v;
        }
    }
    let ss = (0..6).into_iter()
        .map(|n| String::from_iter(&vv[n*40..(n+1)*40]))
        .collect::<Vec<String>>().join("\n");
    Some(ss)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 13140); }

    const P2_RESULT: &str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

    #[test]
    fn test_part_2() { assert_ex!(part_2, P2_RESULT); }
}