use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let input = input
        .trim()
        .lines()
        .map(|line| line.parse::<Point>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let result = part_1(&input);
    println!("{result}");
}

fn part_1(input: &[Point]) -> usize {
    let droplet = input.iter().copied().collect::<HashSet<Point>>();
    let total_sides = input.len() * 6;
    let mut adjacent_sides = 0;
    for point in input {
        for adjacent_point in point.adjacent() {
            if droplet.contains(&adjacent_point) {
                adjacent_sides += 1;
            }
        }
    }
    total_sides - adjacent_sides
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn adjacent(&self) -> [Point; 6] {
        [
            Point {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Point {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Point {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').fuse();
        let x = parts.next().ok_or("Missing x")?;
        let y = parts.next().ok_or("Missing y")?;
        let z = parts.next().ok_or("Missing z")?;
        if parts.next().is_some() {
            return Err("Too many components");
        }
        let x = x.parse().map_err(|_| "Failed to parse x")?;
        let y = y.parse().map_err(|_| "Failed to parse y")?;
        let z = z.parse().map_err(|_| "Failed to parse z")?;
        Ok(Point { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Point>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part_1(&input);
        assert_eq!(result, 64);
    }
}
