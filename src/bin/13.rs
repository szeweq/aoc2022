use std::cmp::Ordering;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

#[derive(Debug, Clone)]
enum Packet {
    N(u32),
    L(Vec<Packet>),
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(u32, |v| Packet::N(v)),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            |v| Packet::L(v),
        ),
    ))(input)
}

fn cmp_packets(left: &Packet, right: &Packet, depth: usize) -> Ordering {
    use Packet::*;
    match (left, right) {
        (N(l), N(r)) => {
            println!("NN {}: {} with {}", depth, l, r);
            l.cmp(r)
        },
        (L(l), N(n)) => {
            println!("LN {}: {:?} with {}", depth, l, n);
            cmp_packets(left, &Packet::L(vec![right.clone()]), depth + 1)
        }
        (N(n), L(l)) => {
            println!("NL {}: {} with {:?}", depth, n, l);
            cmp_packets(&Packet::L(vec![left.clone()]), right, depth + 1)
        }
        (L(l), L(r)) => {
            println!("LL {}: {:?} with {:?}", depth, l, r);
            for (i1, i2) in l.iter().zip(r) {
                let result = cmp_packets(i1, i2, depth + 1);
                if result != Ordering::Equal {
                    return result;
                }
            }
            println!("ZZ {}: len {} with len {}", depth, l.len(), r.len());
            l.len().cmp(&r.len())
        }
    }
}

pub fn part_1(input: &str) -> Option<usize> {
    let mut sum = 0;
    for (i, pair) in input.split("\r\n\r\n").enumerate() {
        let mut lines = pair.lines();
        let left = parse_packet(lines.next().unwrap()).unwrap().1;
        let right = parse_packet(lines.next().unwrap()).unwrap().1;

        if cmp_packets(&left, &right, 0) == Ordering::Less {
            sum += i + 1;
        }
    }
    Some(sum)
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_packet(l).unwrap().1)
        .collect();

    let div1 = Packet::L(vec![Packet::L(vec![Packet::N(2)])]);
    let div2 = Packet::L(vec![Packet::L(vec![Packet::N(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(|a, b| cmp_packets(a, b, 0));
    let mut pos1 = None;
    let mut pos2 = None;
    for (i, p) in packets.iter().enumerate() {
        if pos1.is_none() && cmp_packets(p, &div1, 0) == Ordering::Equal {
            pos1 = Some(i + 1)
        } else if cmp_packets(p, &div2, 0) == Ordering::Equal {
            pos2 = Some(i + 1)
        }
        if pos1.is_some() && pos2.is_some() {
            break;
        }
    }
    Some(pos1.unwrap() * pos2.unwrap())
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 13); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 140); }
}