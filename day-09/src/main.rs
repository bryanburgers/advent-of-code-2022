use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let commands = Command::parse(input).unwrap();
    let result = part_1(&commands);
    println!("{result}");
}

fn part_1(commands: &[Command]) -> usize {
    let mut head = Head(Position::default());
    let mut tail = Tail(Position::default());

    let mut tail_positions = HashSet::new();
    for command in commands {
        let vector = command.unit_vector();
        for _ in 0..command.count() {
            *head.position_mut() += vector;
            tail.move_toward_head(&head);
            tail_positions.insert(tail.position());
        }
    }
    tail_positions.len()
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Add<Vector> for Position {
    type Output = Position;

    fn add(self, rhs: Vector) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

impl AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Position {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vector { x, y }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub const UP: Vector = Vector { x: 0, y: -1 };
    pub const DOWN: Vector = Vector { x: 0, y: 1 };
    pub const LEFT: Vector = Vector { x: -1, y: 0 };
    pub const RIGHT: Vector = Vector { x: 1, y: 0 };
}

#[derive(Debug, Default)]
struct Head(Position);

impl Head {
    pub fn position(&self) -> Position {
        self.0
    }

    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.0
    }
}

#[derive(Debug, Default)]
struct Tail(Position);

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("U ") {
            let num = num.parse().map_err(|_| "Failed to parse")?;
            Ok(Self::Up(num))
        } else if let Some(num) = s.strip_prefix("D ") {
            let num = num.parse().map_err(|_| "Failed to parse")?;
            Ok(Self::Down(num))
        } else if let Some(num) = s.strip_prefix("L ") {
            let num = num.parse().map_err(|_| "Failed to parse")?;
            Ok(Self::Left(num))
        } else if let Some(num) = s.strip_prefix("R ") {
            let num = num.parse().map_err(|_| "Failed to parse")?;
            Ok(Self::Right(num))
        } else {
            Err("Invalid command")
        }
    }
}

impl Command {
    fn count(&self) -> usize {
        match self {
            Command::Left(v) => *v,
            Command::Right(v) => *v,
            Command::Up(v) => *v,
            Command::Down(v) => *v,
        }
    }

    fn unit_vector(&self) -> Vector {
        match self {
            Command::Left(_) => Vector::LEFT,
            Command::Right(_) => Vector::RIGHT,
            Command::Up(_) => Vector::UP,
            Command::Down(_) => Vector::DOWN,
        }
    }

    fn parse(input: &str) -> Result<Vec<Command>, &'static str> {
        input.trim().lines().map(|line| line.parse()).collect()
    }
}

impl Tail {
    pub fn move_toward_head(&mut self, head: &Head) {
        let relative = head.position() - self.position();
        if -1 <= relative.x && relative.x <= 1 && -1 <= relative.y && relative.y <= 1 {
            return;
        }

        self.0.x += relative.x.signum();
        self.0.y += relative.y.signum();
    }

    pub fn position(&self) -> Position {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() {
        let position1 = Position { x: 7, y: 7 };
        let position2 = Position { x: 9, y: 9 };
        let vector = position2 - position1;
        let final_position = position1 + vector;
        assert_eq!(final_position, position2);
    }

    #[test]
    fn move_toward_head() {
        let head = Head(Position { x: 7, y: 7 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 7, y: 7 });

        let head = Head(Position { x: 8, y: 7 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 7, y: 7 });

        let head = Head(Position { x: 7, y: 8 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 7, y: 7 });

        let head = Head(Position { x: 8, y: 8 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 7, y: 7 });

        let head = Head(Position { x: 7, y: 9 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 7, y: 8 });

        let head = Head(Position { x: 9, y: 7 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 8, y: 7 });

        let head = Head(Position { x: 9, y: 8 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 8, y: 8 });

        let head = Head(Position { x: 8, y: 9 });
        let mut tail = Tail(Position { x: 7, y: 7 });
        tail.move_toward_head(&head);
        assert_eq!(tail.position(), Position { x: 8, y: 8 });
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let commands = Command::parse(input).unwrap();
        let result = part_1(&commands);
        assert_eq!(result, 13);
    }
}
