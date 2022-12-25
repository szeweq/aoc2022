/*
 * Day 25: Full of Hot Air
 * See [https://adventofcode.com/2022/day/25]
 */

fn parse_line(line: &str) -> isize {
    let mut sum = 0;
    for l in line.bytes() {
        let lp = match l {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            x => panic!("Invalid character {}", x as char)
        };
        sum *= 5;
        sum += lp;
    }
    sum
}

const NUM_CH: [char; 5] = ['0', '1', '2', '=', '-'];

fn fmt_num(num: isize) -> String {
    let mut wn = num;
    let mut chv = Vec::new();
    while wn > 0 {
        let dp = wn % 5;
        chv.push(NUM_CH[dp as usize]);
        if dp > 2 {
            wn += 5 - dp;
        }
        wn /= 5;
    }
    chv.into_iter().rev().collect()
}

pub fn part_1(input: &str) -> Option<String> {
    let num = input.lines().map(parse_line).sum();
    Some(fmt_num(num))
}

pub fn part_2(_: &str) -> Option<u32> {
    // Part 2 not available for this day
    Some(0)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, "2=-1=0"); }

    //#[test]
    //fn test_part_2() { assert_ex!(part_2, 0); }
}