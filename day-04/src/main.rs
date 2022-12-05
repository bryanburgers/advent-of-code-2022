use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let input = input
        .trim()
        .lines()
        .map(|line| line.parse::<Pair>())
        .collect::<Result<Vec<Pair>, _>>()
        .unwrap();
    let result = part_1(&input);
    println!("{result}");
    let result = part_2(&input);
    println!("{result}");
}

fn part_1(input: &[Pair]) -> usize {
    let mut count = 0;
    for pair in input {
        if pair.first.contains(&pair.second) || pair.second.contains(&pair.first) {
            count += 1;
        }
    }
    count
}

fn part_2(input: &[Pair]) -> usize {
    let mut count = 0;
    for pair in input {
        if pair.first.overlaps(&pair.second) {
            count += 1;
        }
    }
    count
}

pub struct Range {
    min: u64,
    max: u64,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.min <= other.min && other.min <= self.max)
            || (self.min <= other.max && other.max <= self.max)
            || self.contains(other)
            || other.contains(self)
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').ok_or("Missing hyphen")?;
        let min = min.parse().map_err(|_| "Min didn't parse")?;
        let max = max.parse().map_err(|_| "Max didn't parse")?;
        Ok(Range { min, max })
    }
}

pub struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').ok_or("Missing comma")?;
        let first = first.parse().map_err(|_| "First didn't parse")?;
        let second = second.parse().map_err(|_| "First didn't parse")?;
        Ok(Pair { first, second })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part_1() {
        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Pair>())
            .collect::<Result<Vec<Pair>, _>>()
            .unwrap();
        let result = part_1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    pub fn test_part_2() {
        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Pair>())
            .collect::<Result<Vec<Pair>, _>>()
            .unwrap();
        let result = part_2(&input);
        assert_eq!(result, 4);
    }
}
