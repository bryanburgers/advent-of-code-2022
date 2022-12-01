fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_1(input));
}

fn part_1(str: &str) -> u64 {
    let mut current = 0;
    let mut max = 0;
    for line in str.lines() {
        if line.is_empty() {
            max = std::cmp::max(max, current);
            current = 0;
            continue;
        }

        let val = line.parse::<u64>().unwrap();
        current += val;
    }

    max = std::cmp::max(max, current);

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let result = part_1(input);
        assert_eq!(result, 24_000);
    }
}
