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
    let lrows = grid.len();
    let lcols = grid[0].len();
    let mut vmax: usize = 0;

    for j in 1..lrows - 1 {
        let row = grid[j];
        for i in 1..lcols - 1 {
            let el = row[i];
            let rowfn = |&z| el <= z;
            let gridln = |&z: &&[u8]| el <= z[i];

            let t = {
                let vtop = &grid[..j];
                if let Some(x) = vtop.iter().rev().position(gridln) {
                    x + 1
                } else {
                    vtop.len()
                }
            };
            let l = {
                let vleft = &row[..i];
                if let Some(x) = vleft.iter().rev().position(rowfn) {
                    x + 1
                } else {
                    vleft.len()
                }
            };
            let r = {
                let vright = &row[i + 1..];
                if let Some(x) = vright.iter().position(rowfn) {
                    x + 1
                } else {
                    vright.len()
                }
            };
            let b = {
                let vbottom = &grid[j + 1..];
                if let Some(x) = vbottom.iter().position(gridln) {
                    x + 1
                } else {
                    vbottom.len()
                }
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
