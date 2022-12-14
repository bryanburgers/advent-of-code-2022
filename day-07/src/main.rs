mod fs;
mod input;

fn main() {
    let input = input::parse_input(include_str!("input.txt")).unwrap();
    let result = part_1(&input);
    println!("{result}");
    let result = part_2(&input);
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

fn part_2(input: &fs::Entry) -> usize {
    #[derive(Debug)]
    struct PartTwoVisitor {
        current_directory: usize,
        stack: Vec<usize>,
        options: Vec<(fs::Pointer, usize)>,
    }

    impl fs::Visitor for PartTwoVisitor {
        fn visit_directory_before(&mut self, _pointer: &fs::Pointer, _directory: &fs::Directory) {
            self.stack.push(self.current_directory);
            self.current_directory = 0;
        }
        fn visit_directory_after(&mut self, pointer: &fs::Pointer, _directory: &fs::Directory) {
            let size = self.current_directory;
            self.current_directory = self.stack.pop().unwrap_or(0);
            self.current_directory += size;
            self.options.push((pointer.clone(), size));
        }
        fn visit_file(&mut self, _pointer: &fs::Pointer, file: &fs::File) {
            self.current_directory += file.size();
        }
    }

    let mut visitor = PartTwoVisitor {
        current_directory: 0,
        stack: Vec::new(),
        options: Vec::new(),
    };

    input.visit(&mut visitor);

    const DISK_SIZE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;

    let used = visitor.current_directory;
    let free = DISK_SIZE - used;
    if free > REQUIRED_SPACE {
        return 0;
    }
    let need_to_free = REQUIRED_SPACE - free;

    visitor
        .options
        .sort_by(|(_name_a, a), (_name_b, b)| a.cmp(b));

    let (_name, size) = visitor
        .options
        .iter()
        .find(|(_name, size)| *size >= need_to_free)
        .expect("expected at least one directory");

    // println!("{name} {size}");

    *size
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

    #[test]
    fn test_part_2() {
        let input = input::parse_input(include_str!("example.txt")).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 24933642);
    }
}
