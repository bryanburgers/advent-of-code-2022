use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::{fmt::Debug, str::FromStr};

fn main() {
    let input = parse(include_str!("input.txt")).unwrap();
    let result = part_1(&input);
    println!("{result}");
    let result = part_2(&input);
    println!("{result}");
}

fn part_1(input: &[(Packet, Packet)]) -> usize {
    let mut sum = 0;
    for (idx, pair) in input
        .iter()
        .enumerate()
        .map(|(idx, other)| (idx + 1, other))
    {
        let left = &pair.0;
        let right = &pair.1;
        if left.cmp(right) == std::cmp::Ordering::Less {
            sum += idx;
        }
    }
    sum
}

fn part_2(input: &[(Packet, Packet)]) -> usize {
    let mut vec = Vec::with_capacity(input.len() * 2);
    for item in input {
        vec.push(&item.0);
        vec.push(&item.1);
    }
    let marker1 = "[[2]]".parse().unwrap();
    let marker2 = "[[6]]".parse().unwrap();
    vec.push(&marker1);
    vec.push(&marker2);
    vec.sort();

    vec.into_iter()
        .enumerate()
        .map(|(idx, val)| (idx + 1, val))
        .filter(|(_, val)| *val == &marker1 || *val == &marker2)
        .map(|(idx, _)| idx)
        .product()
}

fn parse(input: &str) -> Result<Vec<(Packet, Packet)>, &'static str> {
    // Probably coulda used nom here, too, but meh.
    let lines = input.lines().collect::<Vec<_>>();
    let mut vec = Vec::new();
    for chunk in lines.chunks(3) {
        if chunk.len() < 2 {
            return Err("Unexpected number of lines");
        }
        let packet1 = chunk[0].parse()?;
        let packet2 = chunk[1].parse()?;
        if chunk.len() == 3 && !chunk[2].is_empty() {
            return Err("Expected separator to be empty");
        }
        vec.push((packet1, packet2))
    }
    Ok(vec)
}

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::List(list) => {
                f.write_str("[")?;
                let mut is_first = true;
                for packet in list {
                    if is_first {
                        is_first = false;
                    } else {
                        f.write_str(",")?;
                    }
                    packet.fmt(f)?;
                }
                f.write_str("]")?;
                Ok(())
            }
        }
    }
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn packet(input: &str) -> IResult<&str, Packet> {
            let list = map(list, Packet::List);
            let int = map(int, Packet::Int);
            let mut s = alt((list, int));
            s(input)
        }

        fn list(input: &str) -> IResult<&str, Vec<Packet>> {
            delimited(tag("["), separated_list0(tag(","), packet), tag("]"))(input)
        }

        fn int(input: &str) -> IResult<&str, u8> {
            let (rest, digits) = digit1(input)?;
            Ok((rest, digits.parse().unwrap()))
        }

        let (rest, packet) = packet(s).map_err(|_| "Nope")?;
        if rest.is_empty() {
            Ok(packet)
        } else {
            Err("Incomplete parse")
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::Int(a), b @ Packet::List(_)) => {
                let wrapped = Packet::List(vec![Packet::Int(*a)]);
                wrapped.cmp(b)
            }
            (a @ Packet::List(_), Packet::Int(b)) => {
                let wrapped = Packet::List(vec![Packet::Int(*b)]);
                a.cmp(&wrapped)
            }
            (Packet::List(a), Packet::List(b)) => {
                let a_len = a.len();
                let b_len = b.len();
                for idx in 0..std::cmp::min(a_len, b_len) {
                    let a_item = a.get(idx).unwrap();
                    let b_item = b.get(idx).unwrap();
                    let cmp = a_item.cmp(b_item);
                    if !cmp.is_eq() {
                        return cmp;
                    }
                }
                a_len.cmp(&b_len)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! packet_cmp {
        ($first:literal, $second:literal) => {{
            let first = $first
                .parse::<Packet>()
                .expect("expected first to be parsed");
            let second = $second
                .parse::<Packet>()
                .expect("expected second to be parsed");
            first.cmp(&second)
        }};
    }
    macro_rules! assert_lt {
        ($first:literal, $second:literal) => {
            assert_eq!(packet_cmp!($first, $second), std::cmp::Ordering::Less);
        };
    }
    macro_rules! assert_gt {
        ($first:literal, $second:literal) => {
            assert_eq!(packet_cmp!($first, $second), std::cmp::Ordering::Greater);
        };
    }

    #[test]
    fn test_part_1_examples() {
        assert_lt!("[1,1,3,1,1]", "[1,1,5,1,1]");
        assert_lt!("[[1],[2,3,4]]", "[[1],4]");
        assert_gt!("[9]", "[[8,7,6]]");
        assert_lt!("[[4,4],4,4]", "[[4,4],4,4,4]");
        assert_gt!("[7,7,7,7]", "[7,7,7]");
        assert_lt!("[]", "[3]");
        assert_gt!("[[[]]]", "[[]]");
        assert_gt!("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]");
    }

    #[test]
    fn test_part_1() {
        let input = parse(include_str!("example.txt")).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let input = parse(include_str!("example.txt")).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 140);
    }
}
