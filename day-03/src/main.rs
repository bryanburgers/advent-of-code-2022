use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input.txt");
    let rucksacks: Vec<&Rucksack> = input
        .trim()
        .lines()
        .map(|line| Rucksack::from_slice(line.as_bytes()))
        .collect::<Result<_, _>>()
        .unwrap();

    let part_one = part_one(&rucksacks);
    println!("{part_one}");

    let part_two = part_two(&rucksacks);
    println!("{part_two}");
}

fn part_one(input: &[&Rucksack]) -> usize {
    let mut sum = 0;
    for rucksack in input {
        let common_items = rucksack.common_items();
        assert_eq!(common_items.len(), 1);
        let item = common_items.into_iter().next().unwrap();
        sum += item.priority() as usize;
    }
    sum
}

fn part_two(input: &[&Rucksack]) -> usize {
    let mut sum = 0;

    fn common_items(one: &Rucksack, two: &Rucksack, three: &Rucksack) -> BTreeSet<Item> {
        let set1 = one.as_slice().iter().copied().collect::<BTreeSet<Item>>();
        let set2 = two.as_slice().iter().copied().collect::<BTreeSet<Item>>();
        let set3 = three.as_slice().iter().copied().collect::<BTreeSet<Item>>();

        let int1 = set1
            .intersection(&set2)
            .copied()
            .collect::<BTreeSet<Item>>();
        int1.intersection(&set3).copied().collect()
    }

    for arr in input.chunks(3) {
        assert_eq!(arr.len(), 3);
        let one = arr[0];
        let two = arr[1];
        let three = arr[2];
        let common_items = common_items(one, two, three);
        assert_eq!(common_items.len(), 1);
        let common_item = common_items.into_iter().next().unwrap();
        sum += common_item.priority() as usize;
    }

    sum
}

#[repr(transparent)]
pub struct Rucksack([Item]);

impl Rucksack {
    pub fn from_slice(slice: &[u8]) -> Result<&Rucksack, &'static str> {
        for item in slice {
            if !Item::check(*item) {
                return Err("Invalid input");
            }
        }

        let transmuted = unsafe { std::mem::transmute(slice) };
        Ok(transmuted)
    }

    pub fn as_slice(&self) -> &[Item] {
        &self.0
    }

    pub fn first_compartment(&self) -> &Compartment {
        let slice = self.as_slice();
        let half = slice.len() / 2;
        let slice = &slice[..half];
        Compartment::from_slice(slice)
    }

    pub fn second_compartment(&self) -> &Compartment {
        let slice = self.as_slice();
        let half = slice.len() / 2;
        let slice = &slice[half..];
        Compartment::from_slice(slice)
    }

    pub fn common_items(&self) -> BTreeSet<Item> {
        let mut first = BTreeSet::new();
        let mut second = BTreeSet::new();
        for item in self.first_compartment().as_slice() {
            first.insert(*item);
        }
        for item in self.second_compartment().as_slice() {
            second.insert(*item);
        }
        first.intersection(&second).copied().collect()
    }
}

#[repr(transparent)]
pub struct Compartment([Item]);

impl Compartment {
    pub fn from_slice(slice: &[Item]) -> &Compartment {
        unsafe { std::mem::transmute(slice) }
    }

    pub fn as_slice(&self) -> &[Item] {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Item(u8);

impl Item {
    pub const fn check(input: u8) -> bool {
        input.is_ascii_alphabetic()
    }

    pub fn priority(&self) -> u8 {
        if self.0 >= b'a' && self.0 <= b'z' {
            self.0 - b'a' + 1
        } else if self.0 >= b'A' && self.0 <= b'Z' {
            self.0 - b'A' + 27
        } else {
            unreachable!()
        }
    }
}

impl TryFrom<u8> for Item {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::check(value) {
            Ok(Self(value))
        } else {
            Err("Invalid input")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_sizes() {
        assert_eq!(std::mem::size_of::<u8>(), std::mem::size_of::<Item>());
        assert_eq!(std::mem::size_of::<&[u8]>(), std::mem::size_of::<&[Item]>());
        assert_eq!(
            std::mem::size_of::<&[u8]>(),
            std::mem::size_of::<&Rucksack>()
        );
    }

    #[test]
    fn from_slice() {
        let str = r#"vJrwpWtwJgWrhcsFMMfFFhFp"#;
        let rucksack = Rucksack::from_slice(str.as_bytes()).expect("expect success");
        assert_eq!(rucksack.as_slice()[0], Item::try_from(b'v').unwrap());
    }

    #[test]
    fn compartments() {
        let str = r#"vJrwpWtwJgWrhcsFMMfFFhFp"#;
        let rucksack = Rucksack::from_slice(str.as_bytes()).expect("expect success");
        assert_eq!(rucksack.first_compartment().as_str(), "vJrwpWtwJgWr");
        assert_eq!(rucksack.second_compartment().as_str(), "hcsFMMfFFhFp");
    }

    #[test]
    fn priority() {
        let item = Item::try_from(b'a').unwrap();
        assert_eq!(item.priority(), 1);
        let item = Item::try_from(b'z').unwrap();
        assert_eq!(item.priority(), 26);
        let item = Item::try_from(b'A').unwrap();
        assert_eq!(item.priority(), 27);
        let item = Item::try_from(b'Z').unwrap();
        assert_eq!(item.priority(), 52);
    }

    #[test]
    fn common_items() {
        let str = r#"vJrwpWtwJgWrhcsFMMfFFhFp"#;
        let rucksack = Rucksack::from_slice(str.as_bytes()).expect("expect success");
        assert_eq!(
            rucksack.common_items(),
            BTreeSet::from([Item::try_from(b'p').unwrap()])
        );
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("example.txt");
        let rucksacks: Vec<&Rucksack> = input
            .trim()
            .lines()
            .map(|line| Rucksack::from_slice(line.as_bytes()))
            .collect::<Result<_, _>>()
            .unwrap();

        let part_one = part_one(&rucksacks);
        assert_eq!(part_one, 157);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("example.txt");
        let rucksacks: Vec<&Rucksack> = input
            .trim()
            .lines()
            .map(|line| Rucksack::from_slice(line.as_bytes()))
            .collect::<Result<_, _>>()
            .unwrap();

        let part_two = part_two(&rucksacks);
        assert_eq!(part_two, 70);
    }
}
