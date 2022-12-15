use itertools::Itertools;

fn pts(ch: u8) -> u32 {
    if ch.is_ascii_lowercase() {
        (ch - b'a') as u32 + 1
    } else if ch.is_ascii_uppercase() {
        (ch - b'A') as u32 + 27
    } else { 0 }
}

fn bs_insert(v: &mut Vec<u8>, t: &[u8]) {
    // v.reserve(t.len());
    // for b in t {
    //     if v.is_empty() {
    //         v.push(*b)
    //     } else if let Err(x) = v.binary_search(b) {
    //         v.insert(x, *b)
    //     }
    // }
    v.extend_from_slice(t);
    v.dedup();
    v.sort();
}

pub fn part_1(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    let mut v1: Vec<u8> = Vec::new();
    let mut v2: Vec<u8> = Vec::new();
    for l in lines {
        let lb = l.as_bytes();
        let half = lb.len() / 2;
        let (c1, c2) = lb.split_at(half);
        bs_insert(&mut v1, c1);
        bs_insert(&mut v2, c2);
        let (mut i, mut j) = (0, 0);
        let mut b: u8 = 255;
        loop {
            let u1 = v1[i];
            let u2 = v2[j];
            if u1 == u2 {
                b = u1;
                break;
            } else if u1 > u2 {
                j += 1;
            } else {
                i += 1;
            }
            if i == half || j == half {
                if b == 255 {
                    panic!("No character found!")
                }
                break;
            }
        }
        sum += pts(b);
        v1.clear();
        v2.clear();
    }
    Some(sum)
}

pub fn part_2(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut v1: Vec<u8> = Vec::new();
    let mut v2: Vec<u8> = Vec::new();
    let mut v3: Vec<u8> = Vec::new();
    let lines = input.lines().map(str::as_bytes);
    for (a, b, c) in lines.tuples() {
        bs_insert(&mut v1, a);
        bs_insert(&mut v2, b);
        bs_insert(&mut v3, c);
        let (al, bl, cl) = (v1.len(), v2.len(), v3.len());
        let (mut i, mut j, mut k) = (0, 0, 0);
        let mut by: u8 = 255;
        loop {
            let u1 = v1[i];
            let u2 = v2[j];
            let u3 = v3[k];
            if u1 == u2 && u1 == u3 {
                by = u1;
                break;
            }
            let min = u1.min(u2).min(u3);
            if min == u1 { i += 1; }
            else if min == u2 { j += 1; }
            else if min == u3 { k += 1; }
            if i == al || j == bl || k == cl {
                if by == 255 {
                    panic!("No character found!")
                }
                break;
            }
        }
        sum += pts(by);
        v1.clear();
        v2.clear();
        v3.clear();
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
        assert_ex!(part_1, 157);
    }

    #[test]
    fn test_part_2() {
        assert_ex!(part_2, 70);
    }
}