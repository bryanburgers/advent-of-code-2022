use std::collections::HashMap;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("input.txt");
    let paths = input
        .trim()
        .lines()
        .map(|line| line.parse::<RockPath>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let result = part_1(&paths);
    println!("{result}");
}

fn part_1(paths: &[RockPath]) -> usize {
    let mut cave = Cave::default();
    for path in paths {
        for point in path.clone() {
            cave.set_material(point, Material::Rock);
        }
    }

    let mut resting = 0;
    loop {
        let mut sand = Point { x: 500, y: 0 };
        'sanddrop: loop {
            if sand.y > cave.max_rock() {
                return resting;
            }
            for possible_drop in sand.possible_drops() {
                if cave.get_material(possible_drop).is_none() {
                    sand = possible_drop;
                    continue 'sanddrop;
                }
            }
            // The sand has come to rest!
            cave.set_material(sand, Material::Sand);
            resting += 1;
            break;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn possible_drops(&self) -> impl Iterator<Item = Point> {
        vec![
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
        .into_iter()
    }
}

#[derive(Clone)]
struct RockPath {
    waypoints: Vec<Point>,
}

impl IntoIterator for RockPath {
    type Item = Point;
    type IntoIter = RockPathIterator;

    fn into_iter(self) -> Self::IntoIter {
        RockPathIterator::new(self)
    }
}

struct RockPathIterator {
    current_point: Option<Point>,
    target_point: Option<Point>,
    waypoints: Vec<Point>,
}

impl RockPathIterator {
    pub fn new(path: RockPath) -> Self {
        let mut waypoints = path.waypoints;
        waypoints.reverse();
        let current_point = waypoints.pop();
        let target_point = waypoints.pop();
        Self {
            current_point,
            target_point,
            waypoints,
        }
    }
}

impl Iterator for RockPathIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.current_point?;
        let Some(target_point) = self.target_point else {
            self.current_point = None;
            return Some(point);
        };
        if point == target_point {
            panic!("Unexpected");
        }

        let next_point = Point {
            x: point.x + (target_point.x - point.x).signum(),
            y: point.y + (target_point.y - point.y).signum(),
        };
        if next_point == target_point {
            self.target_point = self.waypoints.pop();
        }

        self.current_point = Some(next_point);

        Some(point)
    }
}

fn parse_rockpath(i: &str) -> IResult<&str, RockPath> {
    let (i, waypoints) = separated_list1(tag(" -> "), parse_point)(i)?;
    let rockpath = RockPath { waypoints };
    Ok((i, rockpath))
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    let (i, (x, y)) = tuple((i32, preceded(tag(","), i32)))(i)?;
    Ok((i, Point { x, y }))
}

impl FromStr for RockPath {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_rockpath(s) {
            Ok(("", rockpath)) => Ok(rockpath),
            Ok(_) => Err("Incomplete parse"),
            Err(_) => Err("Errored parse"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Material {
    Rock,
    Sand,
}

#[derive(Default)]
struct Cave {
    materials: HashMap<Point, Material>,
    max_rock: i32,
}

impl Cave {
    pub fn set_material(&mut self, point: Point, material: Material) {
        if material == Material::Rock && point.y > self.max_rock {
            self.max_rock = point.y;
        }
        self.materials.insert(point, material);
    }

    pub fn get_material(&self, point: Point) -> Option<Material> {
        self.materials.get(&point).copied()
    }

    pub fn max_rock(&self) -> i32 {
        self.max_rock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rockpath() {
        macro_rules! pt {
            ($x:expr, $y:expr) => {
                Point { x: $x, y: $y }
            };
        }

        let input = "498,4 -> 498,6 -> 496,6";
        let rockpath = input.parse::<RockPath>().unwrap();
        let mut iter = rockpath.into_iter();
        assert_eq!(iter.next(), Some(pt!(498, 4)));
        assert_eq!(iter.next(), Some(pt!(498, 5)));
        assert_eq!(iter.next(), Some(pt!(498, 6)));
        assert_eq!(iter.next(), Some(pt!(497, 6)));
        assert_eq!(iter.next(), Some(pt!(496, 6)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let paths = input
            .trim()
            .lines()
            .map(|line| line.parse::<RockPath>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part_1(&paths);
        assert_eq!(result, 24);
    }
}
