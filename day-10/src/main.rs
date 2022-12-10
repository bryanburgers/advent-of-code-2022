use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let input = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let result = part_1(&input);
    println!("{result}");
    let result = part_2(&input);
    println!("{result}");
}

fn part_1(input: &[Instruction]) -> isize {
    let computer = Computer::new(input);
    computer
        .filter(|tick| tick.cycle % 40 == 20 && tick.cycle <= 220)
        .map(|tick| tick.signal_strength())
        .sum()
}

fn part_2(input: &[Instruction]) -> Crt {
    let computer = Computer::new(input);
    let mut crt = Crt::default();
    for tick in computer {
        crt.set(tick.crt_row(), tick.crt_column(), tick.in_sprite());
    }
    crt
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    AddX(isize),
    Noop,
}

impl Instruction {
    fn cycle_count(&self) -> isize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }

    fn apply(&self, value: &mut isize) {
        match self {
            Instruction::AddX(v) => *value += v,
            Instruction::Noop => {}
        }
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some(rest) = s.strip_prefix("addx ") {
            let value = rest.parse().map_err(|_| "Failed to parse number")?;
            Ok(Instruction::AddX(value))
        } else if s == "noop" {
            Ok(Instruction::Noop)
        } else {
            println!("{s}");
            Err("Failed to parse")
        }
    }
}

struct Computer<'a> {
    instructions: &'a [Instruction],
    instruction_pointer: usize,
    instruction_cycle_number: isize,
    cycle: isize,
    value: isize,
}

impl<'a> Debug for Computer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Computer")
            .field("instruction_pointer", &self.instruction_pointer)
            .field("instruction_cycle_number", &self.instruction_cycle_number)
            .field("cycle", &self.cycle)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a> Computer<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            instruction_cycle_number: 0,
            cycle: 1,
            value: 1,
        }
    }

    pub fn tick(&mut self) -> Option<Tick> {
        if self.instruction_pointer >= self.instructions.len() {
            return None;
        }

        let instruction = &self.instructions[self.instruction_pointer];
        let cycle_count = instruction.cycle_count();
        let current_value = self.value;
        let current_cycle = self.cycle;
        self.instruction_cycle_number += 1;
        if self.instruction_cycle_number >= cycle_count {
            instruction.apply(&mut self.value);
            self.instruction_cycle_number = 0;
            self.instruction_pointer += 1;
        }
        self.cycle += 1;

        Some(Tick {
            value: current_value,
            cycle: current_cycle,
        })
    }
}

impl<'a> Iterator for Computer<'a> {
    type Item = Tick;

    fn next(&mut self) -> Option<Self::Item> {
        self.tick()
    }
}

/// The state of the computer *during* a cycle
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Tick {
    cycle: isize,
    value: isize,
}

impl Tick {
    fn signal_strength(&self) -> isize {
        self.cycle * self.value
    }

    fn crt_row(&self) -> isize {
        (self.cycle - 1) / 40
    }

    fn crt_column(&self) -> isize {
        (self.cycle - 1) % 40
    }

    fn in_sprite(&self) -> bool {
        let column = self.crt_column();
        self.value - 1 <= column && column <= self.value + 1
    }
}

struct Crt {
    pixels: [[bool; 40]; 6],
}

impl Crt {
    fn set(&mut self, row: isize, column: isize, value: bool) {
        if row < 0 || row >= 6 || column < 0 || column >= 40 {
            return;
        }
        self.pixels[row as usize][column as usize] = value;
    }
}

impl Default for Crt {
    fn default() -> Self {
        let pixels = [
            [false; 40],
            [false; 40],
            [false; 40],
            [false; 40],
            [false; 40],
            [false; 40],
        ];
        Self { pixels }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                if self.pixels[y][x] {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = r#"
        noop
        addx 3
        addx -5        
        "#;

        let instructions = input
            .trim()
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut computer = Computer::new(&instructions);
        println!("{computer:?}");
        let tick = computer.tick().unwrap();
        assert_eq!(tick.cycle, 1);
        assert_eq!(tick.value, 1);

        println!("{computer:?}");
        let tick = computer.tick().unwrap();
        assert_eq!(tick.cycle, 2);
        assert_eq!(tick.value, 1);

        println!("{computer:?}");
        let tick = computer.tick().unwrap();
        assert_eq!(tick.cycle, 3);
        assert_eq!(tick.value, 1);

        println!("{computer:?}");
        let tick = computer.tick().unwrap();
        assert_eq!(tick.cycle, 4);
        assert_eq!(tick.value, 4);

        println!("{computer:?}");
        let tick = computer.tick().unwrap();
        assert_eq!(tick.cycle, 5);
        assert_eq!(tick.value, 4);

        println!("{computer:?}");

        assert_eq!(computer.tick(), None);
    }

    #[test]
    fn examples() {
        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let computer = Computer::new(&input);
        for tick in computer {
            if tick.cycle == 20 {
                assert_eq!(tick.value, 21);
            }
            if tick.cycle == 60 {
                assert_eq!(tick.value, 19);
            }
            if tick.cycle == 100 {
                assert_eq!(tick.value, 18);
            }
            if tick.cycle == 140 {
                assert_eq!(tick.value, 21);
            }
            if tick.cycle == 180 {
                assert_eq!(tick.value, 16);
            }
            if tick.cycle == 220 {
                assert_eq!(tick.value, 18);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part_1(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part_2() {
        let expected_output = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
        .trim_start();

        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let result = part_2(&input);

        assert_eq!(result.to_string(), expected_output);
    }
}
