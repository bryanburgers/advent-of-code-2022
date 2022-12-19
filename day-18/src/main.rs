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
    let result = part_2(&input);
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

fn part_2(input: &[Point]) -> usize {
    let mut min = Point { x: 1, y: 1, z: 1 };
    let mut max = Point { x: 1, y: 1, z: 1 };
    for point in input {
        min.x = std::cmp::min(point.x, min.x);
        min.y = std::cmp::min(point.y, min.y);
        min.z = std::cmp::min(point.z, min.z);
        max.x = std::cmp::max(point.x, max.x);
        max.y = std::cmp::max(point.y, max.y);
        max.z = std::cmp::max(point.z, max.z);
    }

    min.x -= 1;
    min.y -= 1;
    min.z -= 1;
    max.x += 1;
    max.y += 1;
    max.z += 1;

    let mut count = 0;

    let droplet = input.iter().copied().collect::<HashSet<Point>>();

    let mut seen = HashSet::new();
    let mut stack = vec![min];

    while let Some(item) = stack.pop() {
        if seen.contains(&item) {
            continue;
        }
        seen.insert(item);
        for adjacent in item
            .adjacent()
            .into_iter()
            .filter(|adjacent| adjacent.within_box(min, max))
        {
            if droplet.contains(&adjacent) {
                count += 1;
            } else {
                stack.push(adjacent);
            }
        }
    }

    count
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

    pub fn within_box(&self, min: Point, max: Point) -> bool {
        (min.x <= self.x && self.x <= max.x)
            && (min.y <= self.y && self.y <= max.y)
            && (min.z <= self.z && self.z <= max.z)
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

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let input = input
            .trim()
            .lines()
            .map(|line| line.parse::<Point>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part_2(&input);
        assert_eq!(result, 58);
    }
}
