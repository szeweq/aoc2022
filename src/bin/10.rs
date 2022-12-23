/*
 * Day 10: Cathode-Ray Tube
 * See [https://adventofcode.com/2022/day/10]
 */

use std::fmt::{Display, Write};

#[derive(Debug, PartialEq)]
pub struct MatrixDisplay {
    data: [bool; 240]
}

impl Display for MatrixDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..6 {
            let line = &self.data[i*40..(i+1)*40];
            for j in 0..40 {
                f.write_char(if line[j] { '█' } else { '░' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


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

pub fn part_2(input: &str) -> Option<MatrixDisplay> {
    let mut cycles = 0;
    let mut x = 1i32;
    let mut md = MatrixDisplay { data: [false; 240] };

    macro_rules! cdraw {
        () => {
            let ch = cycles % 40;
            let z = (ch as i32) - x;
            if (-1..2i32).contains(&z) {
                md.data[cycles] = true;
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
    Some(md)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 13140); }

    const T: bool = true;
    const F: bool = false;

    const P2_MD: MatrixDisplay = MatrixDisplay { data: [
        T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F, T, T, F, F,
        T, T, T, F, F, F, T, T, T, F, F, F, T, T, T, F, F, F, T, T, T, F, F, F, T, T, T, F, F, F, T, T, T, F, F, F, T, T, T, F,
        T, T, T, T, F, F, F, F, T, T, T, T, F, F, F, F, T, T, T, T, F, F, F, F, T, T, T, T, F, F, F, F, T, T, T, T, F, F, F, F,
        T, T, T, T, T, F, F, F, F, F, T, T, T, T, T, F, F, F, F, F, T, T, T, T, T, F, F, F, F, F, T, T, T, T, T, F, F, F, F, F,
        T, T, T, T, T, T, F, F, F, F, F, F, T, T, T, T, T, T, F, F, F, F, F, F, T, T, T, T, T, T, F, F, F, F, F, F, T, T, T, T,
        T, T, T, T, T, T, T, F, F, F, F, F, F, F, T, T, T, T, T, T, T, F, F, F, F, F, F, F, T, T, T, T, T, T, T, F, F, F, F, F
    ] };

    #[test]
    fn test_part_2() { assert_ex!(part_2, P2_MD); }
}