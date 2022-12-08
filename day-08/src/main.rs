fn main() {
    let input = include_str!("input.txt");
    let naive = Naive::from_input(input.trim());
    let visible = naive.num_visible();
    println!("{visible}");
    let scenic_score = naive.best_scenic_score();
    println!("{scenic_score}");
}

struct Naive<'a> {
    len: usize,
    data: Vec<&'a [u8]>,
}

impl<'a> Naive<'a> {
    pub fn from_input(val: &'a str) -> Self {
        let mut len = 0;
        let mut data = Vec::new();
        for line in val.lines() {
            len = line.len();
            data.push(line.as_bytes());
        }

        Self { len, data }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn byte_at_point(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.byte_at_point(x, y);

        let mut visible_from_left = true;
        for x in 0..x {
            let h = self.byte_at_point(x, y);
            if h >= height {
                visible_from_left = false;
                break;
            }
        }

        let mut visible_from_right = true;
        for x in (x + 1)..self.len() {
            let h = self.byte_at_point(x, y);
            if h >= height {
                visible_from_right = false;
                break;
            }
        }

        let mut visible_from_above = true;
        for y in 0..y {
            let h = self.byte_at_point(x, y);
            if h >= height {
                visible_from_above = false;
                break;
            }
        }

        let mut visible_from_below = true;
        for y in (y + 1)..self.len() {
            let h = self.byte_at_point(x, y);
            if h >= height {
                visible_from_below = false;
                break;
            }
        }

        visible_from_above || visible_from_below || visible_from_left || visible_from_right
    }

    pub fn num_visible(&self) -> usize {
        let mut count = 0;
        for x in 0..self.len() {
            for y in 0..self.len() {
                if self.is_visible(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.byte_at_point(x, y);
        let mut score_left = 0;
        for x in (0..x).rev() {
            let h = self.byte_at_point(x, y);
            score_left += 1;

            if h >= height {
                break;
            }
        }

        let mut score_right = 0;
        for x in (x + 1)..self.len() {
            let h = self.byte_at_point(x, y);
            score_right += 1;

            if h >= height {
                break;
            }
        }

        let mut score_above = 0;
        for y in (0..y).rev() {
            let h = self.byte_at_point(x, y);
            score_above += 1;

            if h >= height {
                break;
            }
        }

        let mut score_below = 0;
        for y in (y + 1)..self.len() {
            let h = self.byte_at_point(x, y);
            score_below += 1;

            if h >= height {
                break;
            }
        }

        score_left * score_right * score_above * score_below
    }

    pub fn best_scenic_score(&self) -> usize {
        let mut best = 0;
        for x in 0..self.len() {
            for y in 0..self.len() {
                let current = self.scenic_score(x, y);
                if current > best {
                    best = current
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let naive = Naive::from_input(input.trim());
        let visible = naive.num_visible();
        assert_eq!(visible, 21);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let naive = Naive::from_input(input.trim());

        let first_example = naive.scenic_score(2, 1);
        assert_eq!(first_example, 4);

        let second_example = naive.scenic_score(2, 3);
        assert_eq!(second_example, 8);

        let visible = naive.best_scenic_score();
        assert_eq!(visible, 8);
    }
}
