use std::{collections::VecDeque, str::FromStr};

fn main() {
    let monkeys = parse_input(include_str!("input.txt")).unwrap();
    let result = part_1(monkeys.clone());
    println!("{result}");
}

fn parse_input(input: &str) -> Result<Vec<Monkey>, &'static str> {
    let lines = input.lines().collect::<Vec<_>>();
    lines.chunks(7).into_iter().map(Monkey::parse).collect()
}

fn part_1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        round(&mut monkeys);
    }

    let mut number_inspected = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();
    number_inspected.sort_by(|a, b| a.cmp(b).reverse());
    number_inspected.into_iter().take(2).product()
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let throws = monkey.turn();
        for throw in throws {
            let mut other_monkey = monkeys.get_mut(throw.monkey.0).unwrap();
            other_monkey.items.push_back(throw.item);
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: i32,
    if_true: OtherMonkey,
    if_false: OtherMonkey,
    inspected: usize,
}

impl Monkey {
    pub fn turn(&mut self) -> Vec<Throw> {
        let mut throws = Vec::with_capacity(self.items.len());
        while let Some(throw) = self.throw() {
            throws.push(throw);
        }
        throws
    }

    pub fn throw(&mut self) -> Option<Throw> {
        let item = self.items.pop_front()?;
        let item = self.operation.apply(item);
        let item = item.relief();
        let monkey = if item.0 % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        };
        self.inspected += 1;
        Some(Throw { item, monkey })
    }

    fn parse(lines: &[&str]) -> Result<Self, &'static str> {
        if lines.len() < 6 {
            return Err("Must be at least 6 lines");
        }
        if !lines[0].trim().starts_with("Monkey ") {
            return Err("First line must be 'Monkey <n>'");
        }
        let items = lines[1]
            .trim()
            .strip_prefix("Starting items: ")
            .ok_or("Missing 'starting items'")?;
        let operation = lines[2]
            .trim()
            .strip_prefix("Operation: new = ")
            .ok_or("Missing operation")?;
        let test = lines[3]
            .trim()
            .strip_prefix("Test: divisible by ")
            .ok_or("Missing test")?;
        let if_true = lines[4]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .ok_or("Missing if true")?;
        let if_false = lines[5]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .ok_or("MIssing if false")?;
        let items = items
            .split(", ")
            .map(|val| val.parse::<Item>())
            .collect::<Result<VecDeque<_>, _>>()?;
        let operation = operation.parse()?;
        let test = test.parse().map_err(|_| "Failed to parse test")?;
        let if_true = if_true.parse()?;
        let if_false = if_false.parse()?;

        Ok(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspected: 0,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Item(i32);

impl From<i32> for Item {
    fn from(inner: i32) -> Self {
        Self(inner)
    }
}

impl FromStr for Item {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .parse::<i32>()
            .map_err(|_| "Failed to parse item as i32")?;
        Ok(Self(inner))
    }
}

impl Item {
    pub fn relief(self) -> Item {
        Item(self.0 / 3)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct OtherMonkey(usize);

impl From<usize> for OtherMonkey {
    fn from(inner: usize) -> Self {
        Self(inner)
    }
}

impl FromStr for OtherMonkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .parse::<usize>()
            .map_err(|_| "Failed to parse other monkey  as usize")?;
        Ok(Self(inner))
    }
}

#[derive(Copy, Clone)]
enum Operation {
    Add(i32),
    Multiply(i32),
    Square,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old * old" {
            Ok(Self::Square)
        } else if let Some(multiply) = s.strip_prefix("old * ") {
            let multiply = multiply
                .parse()
                .map_err(|_| "Failed to parse multiplication")?;
            Ok(Self::Multiply(multiply))
        } else if let Some(add) = s.strip_prefix("old + ") {
            let add = add.parse().map_err(|_| "Failed to parse addition")?;
            Ok(Self::Add(add))
        } else {
            Err("Invalid operation")
        }
    }
}

impl Operation {
    pub fn apply(&self, item: Item) -> Item {
        match self {
            Operation::Add(val) => Item(item.0 + val),
            Operation::Multiply(val) => Item(item.0 * val),
            Operation::Square => Item(item.0 * item.0),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Throw {
    monkey: OtherMonkey,
    item: Item,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_monkey() {
        let mut monkey = Monkey {
            items: VecDeque::from([Item(79), Item(98)]),
            operation: Operation::Multiply(19),
            test: 23,
            if_true: OtherMonkey(2),
            if_false: OtherMonkey(3),
            inspected: 0,
        };

        let throws = monkey.turn();
        assert_eq!(
            throws,
            vec![
                Throw {
                    item: Item(500),
                    monkey: OtherMonkey(3),
                },
                Throw {
                    item: Item(620),
                    monkey: OtherMonkey(3),
                }
            ]
        );
        assert_eq!(monkey.inspected, 2);
    }

    #[test]
    fn test_part_1() {
        let monkeys = parse_input(include_str!("example.txt")).unwrap();
        let result = part_1(monkeys);
        assert_eq!(result, 10605);
    }
}
