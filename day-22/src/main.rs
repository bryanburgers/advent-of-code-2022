use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

fn main() {
    let input = include_str!("input.txt");
    let input = parser::parse(input);
    let result = part_1(&input);
    println!("{result}");
}

fn part_1(input: &Input) -> u64 {
    let jungle = &input.jungle;
    let mut person = Person::start(jungle);
    for action in &input.actions {
        person.act(*action, jungle);
    }
    person.password()
}

#[derive(Debug, Eq, PartialEq)]
struct Person {
    position: Point,
    direction: Direction,
}

impl Person {
    pub fn start(jungle: &Jungle) -> Self {
        let position = jungle.first_open_space();
        let direction = Direction::East;
        Self {
            position,
            direction,
        }
    }

    pub fn act(&mut self, action: Action, jungle: &Jungle) {
        match action {
            Action::Turn(turn) => self.turn(turn),
            Action::Forward(distance) => self.move_forward(distance, jungle),
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.direction += turn;
    }

    pub fn move_forward(&mut self, distance: usize, jungle: &Jungle) {
        for _ in 0..distance {
            let moved = self.move_forward_one(jungle);
            if !moved {
                break;
            }
        }
    }

    fn move_forward_one(&mut self, jungle: &Jungle) -> bool {
        let (pt, space) = jungle.next_space(self.position, self.direction);
        if space == JungleSpace::Open {
            self.position = pt;
            true
        } else {
            false
        }
    }

    pub fn password(&self) -> u64 {
        let row = self.position.y as u64 + 1;
        let column = self.position.x as u64 + 1;
        let facing = match self.direction {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        };
        1000 * row + 4 * column + facing
    }
}

struct Jungle(Vec<Vec<JungleSpace>>);

impl Debug for Jungle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for space in row {
                match space {
                    JungleSpace::Void => f.write_str(" ")?,
                    JungleSpace::Open => f.write_str(".")?,
                    JungleSpace::Wall => f.write_str("#")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Jungle {
    pub fn first_open_space(&self) -> Point {
        let first = self.0.first().unwrap();
        let x = first
            .iter()
            .position(|space| *space == JungleSpace::Open)
            .unwrap();
        Point { x: x as i32, y: 0 }
    }

    pub fn next_space(&self, point: Point, direction: Direction) -> (Point, JungleSpace) {
        let mut point = point;
        loop {
            let (pt, space) = self.next_space_one(point, direction);
            if space != JungleSpace::Void {
                return (pt, space);
            } else {
                point = pt;
            }
        }
    }

    fn next_space_one(&self, point: Point, direction: Direction) -> (Point, JungleSpace) {
        let mut point = point.in_direction(direction);
        if direction == Direction::North || direction == Direction::South {
            if point.y < 0 {
                point.y = self.0.len() as i32 - 1;
            }
            if point.y >= self.0.len() as i32 {
                point.y = 0;
            }
        }
        if direction == Direction::West || direction == Direction::East {
            if point.x < 0 {
                point.x = self.0[point.y as usize].len() as i32 - 1;
            }
            if point.x >= self.0[point.y as usize].len() as i32 {
                point.x = 0;
            }
        }

        let column = &self.0[point.y as usize];
        if point.x as usize >= column.len() {
            (point, JungleSpace::Void)
        } else {
            let space = column[point.x as usize];
            (point, space)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum JungleSpace {
    Void,
    Open,
    Wall,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn in_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Add<Turn> for Direction {
    type Output = Direction;

    fn add(self, rhs: Turn) -> Self::Output {
        match (self, rhs) {
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,
        }
    }
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, rhs: Turn) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Turn(Turn),
    Forward(usize),
}

struct Input {
    jungle: Jungle,
    actions: Vec<Action>,
}

mod parser {
    use super::*;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::{map, opt};
    use nom::multi::many1;
    use nom::sequence::{preceded, terminated, tuple};
    use nom::IResult;

    pub(super) fn parse(i: &str) -> Input {
        let (rest, input) = input(i).unwrap();
        if !rest.is_empty() {
            panic!("Failed to parse");
        }
        input
    }

    fn input(input: &str) -> IResult<&str, Input> {
        map(
            terminated(
                tuple((jungle, preceded(tag("\n"), actions))),
                opt(tag("\n")),
            ),
            |(jungle, actions)| Input { jungle, actions },
        )(input)
    }

    fn jungle(input: &str) -> IResult<&str, Jungle> {
        map(many1(terminated(jungle_row, tag("\n"))), Jungle)(input)
    }

    fn jungle_row(input: &str) -> IResult<&str, Vec<JungleSpace>> {
        many1(jungle_space)(input)
    }

    fn jungle_space(input: &str) -> IResult<&str, JungleSpace> {
        alt((
            map(tag(" "), |_| JungleSpace::Void),
            map(tag("."), |_| JungleSpace::Open),
            map(tag("#"), |_| JungleSpace::Wall),
        ))(input)
    }

    fn actions(input: &str) -> IResult<&str, Vec<Action>> {
        many1(action)(input)
    }

    fn action(input: &str) -> IResult<&str, Action> {
        alt((forward, map(turn, Action::Turn)))(input)
    }

    fn forward(input: &str) -> IResult<&str, Action> {
        map(u32, |val| Action::Forward(val as usize))(input)
    }

    fn turn(input: &str) -> IResult<&str, Turn> {
        alt((
            map(tag("L"), |_| Turn::Left),
            map(tag("R"), |_| Turn::Right),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jungle() {
        let jungle = Jungle(vec![
            vec![
                JungleSpace::Void,
                JungleSpace::Void,
                JungleSpace::Void,
                JungleSpace::Void,
            ],
            vec![
                JungleSpace::Void,
                JungleSpace::Open,
                JungleSpace::Open,
                JungleSpace::Void,
            ],
            vec![
                JungleSpace::Void,
                JungleSpace::Open,
                JungleSpace::Wall,
                JungleSpace::Void,
            ],
            vec![
                JungleSpace::Void,
                JungleSpace::Void,
                JungleSpace::Void,
                JungleSpace::Void,
            ],
        ]);

        assert_eq!(
            jungle.next_space(Point { x: 1, y: 1 }, Direction::East),
            (Point { x: 2, y: 1 }, JungleSpace::Open)
        );
        assert_eq!(
            jungle.next_space(Point { x: 2, y: 1 }, Direction::East),
            (Point { x: 1, y: 1 }, JungleSpace::Open)
        );
        assert_eq!(
            jungle.next_space(Point { x: 2, y: 1 }, Direction::North),
            (Point { x: 2, y: 2 }, JungleSpace::Wall)
        );
        assert_eq!(
            jungle.next_space(Point { x: 2, y: 1 }, Direction::South),
            (Point { x: 2, y: 2 }, JungleSpace::Wall)
        );
    }

    #[test]
    fn test_person() {
        let jungle = Jungle(vec![
            vec![
                JungleSpace::Void,
                JungleSpace::Open,
                JungleSpace::Wall,
                JungleSpace::Void,
            ],
            vec![
                JungleSpace::Void,
                JungleSpace::Open,
                JungleSpace::Open,
                JungleSpace::Void,
            ],
            vec![
                JungleSpace::Void,
                JungleSpace::Void,
                JungleSpace::Void,
                JungleSpace::Void,
            ],
        ]);

        let mut person = Person::start(&jungle);
        assert_eq!(person.position, Point { x: 1, y: 0 });
        person.move_forward(1, &jungle);
        assert_eq!(person.position, Point { x: 1, y: 0 });
        person.turn(Turn::Left);
        assert_eq!(person.direction, Direction::North);
        person.move_forward(1, &jungle);
        assert_eq!(person.position, Point { x: 1, y: 1 });
        person.turn(Turn::Right);
        person.move_forward(1, &jungle);
        assert_eq!(person.position, Point { x: 2, y: 1 });
        person.turn(Turn::Right);
        assert_eq!(person.direction, Direction::South);
        person.move_forward(1, &jungle);
        assert_eq!(person.position, Point { x: 2, y: 1 });
        person.move_forward(100, &jungle);
        assert_eq!(person.position, Point { x: 2, y: 1 });
    }

    fn get_test_jungle() -> Jungle {
        let input = super::parser::parse(include_str!("test.txt"));
        input.jungle
    }

    fn test(jungle: &Jungle, mut start_person: Person, actions: Vec<Action>, end_person: Person) {
        for action in actions {
            start_person.act(action, jungle);
        }
        assert_eq!(start_person, end_person);
    }

    #[test]
    fn test_wraparound_east_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 3 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 10, y: 3 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_west_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 3 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 10, y: 3 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_east_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 2 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 12, y: 2 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_west_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 1 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 8, y: 1 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_east_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 1 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 11, y: 1 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_west_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 2 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 9, y: 2 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_east_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 7 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 6, y: 7 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_west_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 7 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 6, y: 7 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_east_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 6 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 8, y: 6 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_west_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 5 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 4, y: 5 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_east_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 5 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 7, y: 5 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_west_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 6 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 5, y: 6 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_east_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 11 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 2, y: 11 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_west_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 11 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 2, y: 11 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_east_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 10 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 4, y: 10 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_west_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 9 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 0, y: 9 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_east_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 9 },
            direction: Direction::East,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 3, y: 9 },
            direction: Direction::East,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_west_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 10 },
            direction: Direction::West,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 1, y: 10 },
            direction: Direction::West,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_south_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 9, y: 2 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 9, y: 2 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_north_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 9, y: 2 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 9, y: 2 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_south_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 11, y: 2 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 11, y: 4 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_north_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 2 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 10, y: 0 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_south_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 10, y: 2 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 10, y: 3 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_north_quadrant_ur() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 11, y: 2 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 11, y: 1 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_south_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 5, y: 6 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 5, y: 6 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_north_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 5, y: 6 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 5, y: 6 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_south_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 7, y: 6 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 7, y: 8 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_north_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 6 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 6, y: 4 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_south_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 6, y: 6 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 6, y: 7 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_north_quadrant_mid() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 7, y: 6 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 7, y: 5 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_south_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 1, y: 10 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 1, y: 10 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_wraparound_north_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 1, y: 10 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 1, y: 10 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_south_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 3, y: 10 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 3, y: 12 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_cant_wraparound_north_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 10 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 2, y: 8 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_south_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 2, y: 10 },
            direction: Direction::South,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 2, y: 11 },
            direction: Direction::South,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_regular_block_north_quadrant_br() {
        let jungle = get_test_jungle();
        let start = Person {
            position: Point { x: 3, y: 10 },
            direction: Direction::North,
        };
        let actions = vec![Action::Forward(5)];
        let end = Person {
            position: Point { x: 3, y: 9 },
            direction: Direction::North,
        };
        test(&jungle, start, actions, end);
    }

    #[test]
    fn test_part_1() {
        let input = super::parser::parse(include_str!("example.txt"));
        let result = part_1(&input);
        assert_eq!(result, 6032);
    }
}
