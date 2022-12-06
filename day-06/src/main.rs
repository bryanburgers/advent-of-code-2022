fn main() {
    let input = include_str!("input.txt").trim();
    let result = part_1(input);
    println!("{result}");
    let result = part_2(input);
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
}
