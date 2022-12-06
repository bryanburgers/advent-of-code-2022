fn main() {
    let input = include_str!("input.txt").trim();
    let result = solve_with_scanner(input, 4);
    println!("{result}");
    let result = solve_with_scanner(input, 14);
    println!("{result}");
}

fn part_1(str: &str) -> usize {
    let len = str.len();
    for end in 4..len {
        let start = end - 4;
        let substr = &str[start..end];
        if is_valid_quad(substr) {
            return end;
        }
    }
    panic!()
}

#[inline(always)]
fn is_valid_quad(str: &str) -> bool {
    // Assumes the string is 4 characters long!
    let bytes = str.as_bytes();
    bytes[0] != bytes[1]
        && bytes[0] != bytes[2]
        && bytes[0] != bytes[3]
        && bytes[1] != bytes[2]
        && bytes[1] != bytes[3]
        && bytes[2] != bytes[3]
}

fn part_2(str: &str) -> usize {
    let len = str.len();
    for end in 14..len {
        let start = end - 14;
        let substr = &str[start..end];
        if is_valid_start_of_message(substr) {
            return end;
        }
    }
    panic!()
}

#[inline(always)]
fn is_valid_start_of_message(str: &str) -> bool {
    // Assumes the string is 14 characters long!
    let bytes = str.as_bytes();
    for i in 0..14 {
        for j in (i + 1)..14 {
            if bytes[i] == bytes[j] {
                return false;
            }
        }
    }
    true
}

struct Scanner<'a> {
    bytes: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Scanner<'a> {
    pub fn from_str(str: &'a str) -> Self {
        Self {
            bytes: str.as_bytes(),
            start: 0,
            end: 0,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = std::ops::Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.end += 1;
        if self.end > self.bytes.len() {
            return None;
        }

        let end_byte = self.bytes[self.end - 1];
        for idx in self.start..self.end - 1 {
            if self.bytes[idx] == end_byte {
                self.start = idx + 1;
            }
        }

        Some(self.start..self.end)
    }
}

fn solve_with_scanner(str: &str, size: usize) -> usize {
    let scanner = Scanner::from_str(str);
    for range in scanner {
        if range.end - range.start >= size {
            return range.end;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn scanner() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let mut scanner = Scanner::from_str("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(scanner.next().map(|range| &input[range]), Some("m"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("mj"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("mjq"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("qj"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("qjp"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("jpq"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("jpqm"));
        assert_eq!(scanner.next().map(|range| &input[range]), Some("jpqmg"));

        assert_eq!(
            scanner.last().map(|range| &input[range]),
            Some("phdztnvjfqwrcgsmlb")
        )
    }

    #[test]
    fn test_part_1_with_scanner() {
        assert_eq!(solve_with_scanner("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(solve_with_scanner("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(solve_with_scanner("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            solve_with_scanner("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            solve_with_scanner("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }

    #[test]
    fn test_part_2_with_scanner() {
        assert_eq!(solve_with_scanner("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(solve_with_scanner("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solve_with_scanner("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            solve_with_scanner("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            solve_with_scanner("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
