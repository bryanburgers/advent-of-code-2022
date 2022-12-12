use std::{collections::VecDeque, fmt::Debug, str::FromStr};

fn main() {
    let monkeys = parse_input(include_str!("input.txt")).unwrap();
    let result = part_1(monkeys.clone());
    println!("{result}");
    let result = part_2(monkeys);
    println!("{result}");
}

fn parse_input(input: &str) -> Result<Vec<Monkey>, &'static str> {
    let lines = input.lines().collect::<Vec<_>>();
    lines.chunks(7).into_iter().map(Monkey::parse).collect()
}

fn part_1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        round(&mut monkeys, Worry::Regular);
    }

    let mut number_inspected = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();
    number_inspected.sort_by(|a, b| a.cmp(b).reverse());
    number_inspected.into_iter().take(2).product()
}

fn part_2(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..10_000 {
        round(&mut monkeys, Worry::Extreme);
    }

    let mut number_inspected = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();
    number_inspected.sort_by(|a, b| a.cmp(b).reverse());
    number_inspected.into_iter().take(2).product()
}

fn round(monkeys: &mut Vec<Monkey>, worry_level: Worry) {
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let throws = monkey.turn(worry_level);
        for throw in throws {
            let other_monkey = monkeys.get_mut(throw.monkey.0).unwrap();
            other_monkey.items.push_back(throw.item);
        }
    }
}

#[derive(Copy, Clone)]
enum Worry {
    /// Your worry level divides by 3 after each inspection
    Regular,
    /// Your worry level does not divide by 3 after each inspection
    Extreme,
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: i64,
    if_true: OtherMonkey,
    if_false: OtherMonkey,
    inspected: usize,
}

impl Monkey {
    pub fn turn(&mut self, worry_level: Worry) -> Vec<Throw> {
        let mut throws = Vec::with_capacity(self.items.len());
        while let Some(throw) = self.throw(worry_level) {
            throws.push(throw);
        }
        throws
    }

    pub fn throw(&mut self, worry_level: Worry) -> Option<Throw> {
        let item = self.items.pop_front()?;
        let mut item = self.operation.apply(item);
        if matches!(worry_level, Worry::Regular) {
            item = item.relief();
        } else {
            item = item.manageable();
        }
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
struct Item(i64);

impl From<i64> for Item {
    fn from(inner: i64) -> Self {
        Self(inner)
    }
}

impl FromStr for Item {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .parse::<i64>()
            .map_err(|_| "Failed to parse item as i64")?;
        Ok(Self(inner))
    }
}

impl Item {
    const MOD: i64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

    pub fn relief(self) -> Item {
        Item(self.0 / 3)
    }

    pub fn manageable(self) -> Item {
        Item(self.0 % Self::MOD)
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
    Add(i64),
    Multiply(i64),
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

        let throws = monkey.turn(Worry::Regular);
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

    #[test]
    fn test_part_2_after_1_round() {
        let mut monkeys = parse_input(include_str!("example.txt")).unwrap();
        round(&mut monkeys, Worry::Extreme);
        assert_eq!(monkeys[0].inspected, 2);
        assert_eq!(monkeys[1].inspected, 4);
        assert_eq!(monkeys[2].inspected, 3);
        assert_eq!(monkeys[3].inspected, 6);
    }

    #[test]
    fn test_part_2_after_20_rounds() {
        let mut monkeys = parse_input(include_str!("example.txt")).unwrap();
        for _ in 0..20 {
            round(&mut monkeys, Worry::Extreme);
        }
        assert_eq!(monkeys[0].inspected, 99);
        assert_eq!(monkeys[1].inspected, 97);
        assert_eq!(monkeys[2].inspected, 8);
        assert_eq!(monkeys[3].inspected, 103);
    }

    #[test]
    fn test_part_2_after_1000_rounds() {
        let mut monkeys = parse_input(include_str!("example.txt")).unwrap();
        for _ in 0..1000 {
            round(&mut monkeys, Worry::Extreme);
        }
        assert_eq!(monkeys[0].inspected, 5204);
        assert_eq!(monkeys[1].inspected, 4792);
        assert_eq!(monkeys[2].inspected, 199);
        assert_eq!(monkeys[3].inspected, 5192);
    }

    #[test]
    fn test_part_2_after_7000_rounds() {
        let mut monkeys = parse_input(include_str!("example.txt")).unwrap();
        for _ in 0..7_000 {
            round(&mut monkeys, Worry::Extreme);
        }
        assert_eq!(monkeys[0].inspected, 36508);
        assert_eq!(monkeys[1].inspected, 33488);
        assert_eq!(monkeys[2].inspected, 1360);
        assert_eq!(monkeys[3].inspected, 36400);
    }

    #[test]
    fn test_part_2() {
        let monkeys = parse_input(include_str!("example.txt")).unwrap();
        let result = part_2(monkeys);
        assert_eq!(result, 2713310158);
    }
}
