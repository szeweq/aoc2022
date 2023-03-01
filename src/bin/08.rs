/*
 * Day 8: Treetop Tree House
 * See [https://adventofcode.com/2022/day/8]
 */

fn input_grid(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect::<Vec<&[u8]>>()
}

pub fn part_1(input: &str) -> Option<usize> {
    let grid = input_grid(input);
    let lh = grid.len() - 1;
    let lw = grid[0].len() - 1;
    let mut sum = (lw + lh) * 2;

    for j in 1..lh {
        let row = grid[j];
        for i in 1..lw {
            let el = row[i];
            if el == b'0' {
                continue;
            }
            if grid[..j].iter().rev().all(|z| el > z[i])
            || grid[j+1..].iter().all(|z| el > z[i])
            || row[..i].iter().rev().all(|z| el > *z)
            || row[i+1..].iter().all(|z| el > *z) {
                sum += 1;
            }
        }
    }
    Some(sum)
}

pub fn part_2(input: &str) -> Option<usize> {
    let grid = input_grid(input);
    let lh = grid.len() - 1;
    let lw = grid[0].len() - 1;
    let mut vmax: usize = 0;

    for j in 1..lh {
        let row = grid[j];
        for i in 1..lw {
            let el = row[i];
            let rowfn = |&z| el <= z;
            let gridln = |&z: &&[u8]| el <= z[i];

            let t = if let Some(x) = &grid[..j].iter().rev().position(gridln) {
                x + 1
            } else {
                j
            };
            let l = if let Some(x) = &row[..i].iter().rev().position(rowfn) {
                x + 1
            } else {
                i
            };
            let r = if let Some(x) = &row[i + 1..].iter().position(rowfn) {
                x + 1
            } else {
                lh - i
            };
            let b = if let Some(x) = &grid[j + 1..].iter().position(gridln) {
                x + 1
            } else {
                lw - j
            };

            let val = t * l * r * b;
            if vmax < val {
                vmax = val;
            }
        }
    }

    Some(vmax)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::assert_ex;

    #[test]
    fn test_part_1() {
        assert_ex!(part_1, 21);
    }

    #[test]
    fn test_part_2() {
        assert_ex!(part_2, 8);
    }
}
