use std::collections::HashMap;

fn main() {
    let grid = Grid::parse(include_str!("input.txt")).unwrap();
    let result = part_1(&grid);
    println!("{result}");
}

fn part_1(grid: &Grid) -> usize {
    let pathfinder = Pathfinder::new(grid);
    let mut smallest = grid.width * grid.height + 2;
    for path in pathfinder {
        let steps = path.len() - 1;
        if steps < smallest {
            smallest = steps
        }
    }
    smallest
}

struct Grid {
    start: Point,
    end: Point,
    inner: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn at(&self, point: Point) -> u8 {
        self.inner[point.y as usize][point.x as usize]
    }

    pub fn contains(&self, point: Point) -> bool {
        0 <= point.x
            && (point.x as usize) < self.width
            && 0 <= point.y
            && (point.y as usize) < self.height
    }

    pub fn move_allowed(&self, from: Point, to: Point) -> bool {
        if !self.contains(from) || !self.contains(to) {
            return false;
        }

        if !(((from.x - to.x).abs() == 1 && (from.y - to.y) == 0)
            || ((from.y - to.y).abs() == 1 && (from.x - to.x) == 0))
        {
            return false;
        }

        let from = self.at(from);
        let to = self.at(to);
        from > to || to - from <= 1
    }

    fn parse(input: &str) -> Result<Self, &'static str> {
        let mut start = None;
        let mut end = None;
        let mut inner = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut v = line.as_bytes().to_vec();
            for (x, byte) in v.iter_mut().enumerate() {
                if *byte == b'S' {
                    *byte = b'a';
                    start = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                if *byte == b'E' {
                    *byte = b'z';
                    end = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
            inner.push(v)
        }

        let start = start.ok_or("Missing start")?;
        let end = end.ok_or("Missing end")?;
        let height = inner.len();
        if height == 0 {
            return Err("Zero height")?;
        }
        let width = inner[0].len();
        Ok(Self {
            inner,
            start,
            end,
            width,
            height,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn in_direction(self, direction: Direction) -> Self {
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

struct Pathfinder<'a> {
    grid: &'a Grid,
    shortest_path_so_far: HashMap<Point, usize>,
    stack: Vec<(Point, DirectionIterator)>,
}

impl<'a> Pathfinder<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            shortest_path_so_far: HashMap::from([(grid.start, 1)]),
            stack: vec![(grid.start, DirectionIterator::default())],
        }
    }
}

impl<'a> Iterator for Pathfinder<'a> {
    type Item = Vec<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current_stack_length = self.stack.len();
            let last_mut = self.stack.last_mut()?;
            let next_direction = last_mut.1.next();
            let Some(next_direction) = next_direction else {
                self.stack.pop();
                continue;
            };
            let next_point = last_mut.0.in_direction(next_direction);
            let shortest_path_so_far = self.shortest_path_so_far.get(&next_point).copied();
            if shortest_path_so_far.is_some()
                && shortest_path_so_far.unwrap() <= current_stack_length + 1
            {
                continue;
            }
            if !self.grid.move_allowed(last_mut.0, next_point) {
                continue;
            }
            if next_point == self.grid.end {
                let mut path = self.stack.iter().map(|(pt, _)| *pt).collect::<Vec<_>>();
                path.push(next_point);
                return Some(path);
            }

            self.shortest_path_so_far
                .insert(next_point, current_stack_length + 1);
            // self.seen_points.insert(next_point);
            self.stack.push((next_point, DirectionIterator::default()));
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct DirectionIterator {
    next_direction: Option<Direction>,
}

impl Default for DirectionIterator {
    fn default() -> Self {
        Self {
            next_direction: Some(Direction::North),
        }
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_direction == Some(Direction::North) {
            self.next_direction = Some(Direction::South);
            Some(Direction::North)
        } else if self.next_direction == Some(Direction::South) {
            self.next_direction = Some(Direction::West);
            Some(Direction::South)
        } else if self.next_direction == Some(Direction::West) {
            self.next_direction = Some(Direction::East);
            Some(Direction::West)
        } else if self.next_direction == Some(Direction::East) {
            self.next_direction = None;
            Some(Direction::East)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let grid = Grid::parse(include_str!("example.txt")).unwrap();
        let steps = part_1(&grid);
        assert_eq!(steps, 31);
    }
}
