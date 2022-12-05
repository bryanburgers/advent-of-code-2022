use std::{collections::VecDeque, str::FromStr};

macro_rules! stack {
    ($($lit:literal,)*) => {
        stack!($($lit),*)
    };
    ($($lit:literal),*) => {
        {
            let mut vec = VecDeque::new();
            $(vec.push_back($lit);)*
            Stack(vec)
        }
    };
}

fn main() {
    let dock = Dock::from_stacks([
        stack!('C', 'S', 'G', 'B'),
        stack!('G', 'V', 'N', 'J', 'H', 'W', 'M', 'T'),
        stack!('S', 'Q', 'M'),
        stack!('M', 'N', 'W', 'T', 'L', 'S', 'B'),
        stack!('P', 'W', 'G', 'V', 'T', 'F', 'Z', 'J'),
        stack!('S', 'H', 'Q', 'G', 'B', 'T', 'C'),
        stack!('W', 'B', 'P', 'J', 'T'),
        stack!('M', 'Q', 'T', 'F', 'Z', 'C', 'D', 'G'),
        stack!('F', 'P', 'B', 'H', 'S', 'N'),
    ]);
    let input = include_str!("input.txt");
    let commands = parse_commands(input);
    let result = part_1(dock.clone(), &commands);
    println!("{result}");
    let result = part_2(dock, &commands);
    println!("{result}");
}

fn part_1(mut dock: Dock, commands: &[Command]) -> String {
    for command in commands {
        dock.handle_command(command);
    }
    dock.tops()
}

fn part_2(mut dock: Dock, commands: &[Command]) -> String {
    for command in commands {
        dock.handle_command_v2(command);
    }
    dock.tops()
}

#[derive(Clone)]
struct Dock {
    stacks: Vec<Stack>,
}

impl Dock {
    pub fn move_one(&mut self, source: usize, destination: usize) {
        let source_stack = &mut self.stacks[source];
        let c = source_stack.pop();
        let destination_stack = &mut self.stacks[destination];
        destination_stack.push(c);
    }

    pub fn handle_command(&mut self, command: &Command) {
        let source = command.source;
        let destination = command.destination;
        for _ in 0..command.count {
            self.move_one(source, destination);
        }
    }

    pub fn move_multiple_v2(&mut self, source: usize, destination: usize, count: usize) {
        let source_stack = &mut self.stacks[source];
        let on_crane = source_stack.pop_stack(count);
        let dest_stack = &mut self.stacks[destination];
        dest_stack.push_stack(on_crane);
    }

    pub fn handle_command_v2(&mut self, command: &Command) {
        let source = command.source;
        let destination = command.destination;
        self.move_multiple_v2(source, destination, command.count);
    }

    pub fn tops(&self) -> String {
        let mut string = String::new();
        for stack in &self.stacks {
            string.push(stack.peek());
        }
        string
    }

    pub fn from_stacks<const C: usize>(stacks: [Stack; C]) -> Dock {
        let stacks: Vec<Stack> = Vec::from(stacks);
        Self { stacks }
    }
}

#[derive(Debug, Clone)]
struct Stack(VecDeque<char>);

impl Stack {
    pub fn push(&mut self, c: char) {
        self.0.push_front(c);
    }

    pub fn pop(&mut self) -> char {
        self.0.pop_front().unwrap()
    }

    pub fn pop_stack(&mut self, size: usize) -> Stack {
        let mut queue = VecDeque::with_capacity(size);
        for _ in 0..size {
            queue.push_back(self.pop());
        }
        Stack(queue)
    }

    pub fn push_stack(&mut self, mut stack: Stack) {
        while let Some(c) = stack.0.pop_back() {
            self.push(c)
        }
    }

    pub fn peek(&self) -> char {
        *self.0.front().unwrap()
    }
}

#[derive(Debug)]
struct Command {
    count: usize,
    source: usize,
    destination: usize,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp = s.strip_prefix("move ").ok_or("Didn't start with 'move'")?;
        let (num, rest) = tmp.split_once(' ').ok_or("No space")?;
        let count = num.parse::<usize>().map_err(|_| "Failed to parse count")?;
        let tmp = rest.strip_prefix("from ").ok_or("Didn't include 'from'")?;
        let (src, rest) = tmp.split_once(' ').ok_or("No space")?;
        let source = src.parse::<usize>().map_err(|_| "Failed to parse source")? - 1;
        let tmp = rest.strip_prefix("to ").ok_or("Didn't include 'to'")?;
        let destination = tmp.parse::<usize>().map_err(|_| "Failed to parse dest")? - 1;
        Ok(Command {
            count,
            source,
            destination,
        })
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .filter_map(|line| line.parse::<Command>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let commands = parse_commands(input);
        let dock = Dock::from_stacks([stack!('N', 'Z'), stack!('D', 'C', 'M'), stack!('P')]);

        let result = part_1(dock, &commands);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let commands = parse_commands(input);
        let dock = Dock::from_stacks([stack!('N', 'Z'), stack!('D', 'C', 'M'), stack!('P')]);

        let result = part_2(dock, &commands);
        assert_eq!(result, "MCD");
    }
}
