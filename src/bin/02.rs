#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock, Paper, Scissors
}

#[derive(PartialEq, Clone, Copy)]
enum PlayResult {
    Lose, Draw, Win
}

fn parse_rps(b: u8) -> RPS {
    use RPS::*;
    match b {
        b'A' | b'X' => Rock,
        b'B' | b'Y' => Paper,
        b'C' | b'Z' => Scissors,
        _ => panic!("Unknown character for type")
    }
}

fn parse_result(b: u8) -> PlayResult {
    use PlayResult::*;
    match b {
        b'X' => Lose,
        b'Y' => Draw,
        b'Z' => Win,
        _ => panic!("Unknown character for type")
    }
}

macro_rules! beat {
    ($e:expr, $w:expr) => { if $e == $w { Win } else { Lose } };
}

fn play_rps(you: RPS, enemy: RPS) -> PlayResult {
    use RPS::*;
    use PlayResult::*;
    if you == enemy {
        Draw
    } else {
        match you {
            Rock => beat!(enemy, Scissors),
            Paper => beat!(enemy, Rock),
            Scissors => beat!(enemy, Paper)
        }
    }
}

fn guess_rps(enemy: RPS, pr: PlayResult) -> RPS {
    use RPS::*;
    use PlayResult::*;
    match pr {
        Lose => match enemy {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Draw => enemy,
        Win => match enemy {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

fn pts(you: RPS, pr: PlayResult) -> u32 {
    use RPS::*;
    use PlayResult::*;
    let a = match you {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    };
    let b = match pr {
        Lose => 0,
        Draw => 3,
        Win => 6
    };
    a + b
}


pub fn part_1(input: &str) -> Option<u32> {
    let mut sum = 0;
    for l in input.lines() {
        if l.is_empty() { continue; }
        let bl = l.as_bytes();
        let (abc, xyz) = (bl[0], bl[2]);
        let ce = parse_rps(abc);
        let cu = parse_rps(xyz);
        let pr = play_rps(cu, ce);
        sum += pts(cu, pr);
    }
    Some(sum)
}

pub fn part_2(input: &str) -> Option<u32> {
    let mut sum = 0;
    for l in input.lines() {
        if l.is_empty() { continue; }
        let bl = l.as_bytes();
        let (abc, xyz) = (bl[0], bl[2]);
        let ce = parse_rps(abc);
        let cp = parse_result(xyz);
        let u = guess_rps(ce, cp);
        sum += pts(u, cp);
    }
    Some(sum)
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::assert_ex;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 15); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 12); }
}