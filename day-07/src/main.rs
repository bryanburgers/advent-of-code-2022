mod fs;
mod input;

fn main() {
    let input = input::parse_input(include_str!("input.txt")).unwrap();
    let result = part_1(&input);
    println!("{result}");
}

fn part_1(input: &fs::Entry) -> usize {
    struct PartOneVisitor {
        current_directory: usize,
        stack: Vec<usize>,
        total: usize,
    }

    impl fs::Visitor for PartOneVisitor {
        fn visit_directory_before(&mut self, _pointer: &fs::Pointer, _directory: &fs::Directory) {
            self.stack.push(self.current_directory);
            self.current_directory = 0;
        }
        fn visit_directory_after(&mut self, _pointer: &fs::Pointer, _directory: &fs::Directory) {
            let size = self.current_directory;
            self.current_directory = self.stack.pop().unwrap_or(0);
            self.current_directory += size;
            if size <= 100_000 {
                self.total += size;
            }
        }
        fn visit_file(&mut self, _pointer: &fs::Pointer, file: &fs::File) {
            self.current_directory += file.size();
        }
    }

    let mut visitor = PartOneVisitor {
        current_directory: 0,
        stack: Vec::new(),
        total: 0,
    };

    input.visit(&mut visitor);

    visitor.total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = input::parse_input(include_str!("example.txt")).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 95437);
    }
}
