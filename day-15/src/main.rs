#![allow(unused)]

use std::{collections::HashMap, fmt::Debug, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let pairs = input
        .trim()
        .lines()
        .map(|line| line.parse::<Pair>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let result = part_1(&pairs, 2_000_000);
    println!("{result}");
}

fn part_1(input: &[Pair], row: i32) -> usize {
    let mut grid = GridRow::new(row);
    for pair in input {
        let sensor = pair.sensor;
        let beacon = pair.beacon;
        grid.insert(sensor.point(), Occupied::Sensor);
        grid.insert(beacon.point(), Occupied::Beacon);
        for pt in sensor.covered_area_on_row(beacon, row) {
            grid.insert(pt, Occupied::Covered);
        }
    }

    grid.items
        .into_iter()
        // .filter(|(pt, _)| pt.y == row)
        .filter(|(_, occupied)| *occupied != Occupied::Beacon)
        .count()
}

#[derive(Copy, Clone)]
struct Sensor(Point);

impl From<Point> for Sensor {
    fn from(point: Point) -> Self {
        Self(point)
    }
}

impl Sensor {
    pub fn point(&self) -> Point {
        self.0
    }

    fn distance_to_point(&self, point: Point) -> i32 {
        self.point().manhatten_distance_to(point)
    }

    fn distance_to_beacon(&self, beacon: Beacon) -> i32 {
        self.distance_to_point(beacon.point())
    }

    fn covered_area(&self, beacon: Beacon) -> impl Iterator<Item = Point> {
        let manhatten_distance = self.distance_to_beacon(beacon);
        CoveredAreaIterator::new(*self, manhatten_distance)
    }

    fn covered_area_on_row(&self, beacon: Beacon, row: i32) -> impl Iterator<Item = Point> {
        let manhatten_distance = self.distance_to_beacon(beacon);
        CoveredAreaOnRowIterator::new(*self, manhatten_distance, row)
    }
}

#[derive(Copy, Clone)]
struct Beacon(Point);

impl From<Point> for Beacon {
    fn from(point: Point) -> Self {
        Self(point)
    }
}

impl Beacon {
    pub fn point(&self) -> Point {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.x.fmt(f)?;
        f.write_str(",")?;
        self.y.fmt(f)?;
        f.write_str(")")?;
        Ok(())
    }
}

impl Point {
    fn manhatten_distance_to(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn up(self, distance: i32) -> Self {
        Self {
            x: self.x,
            y: self.y - distance,
        }
    }

    fn down(self, distance: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + distance,
        }
    }

    fn left(self, distance: i32) -> Self {
        Self {
            x: self.x - distance,
            y: self.y,
        }
    }

    fn right(self, distance: i32) -> Self {
        Self {
            x: self.x + distance,
            y: self.y,
        }
    }
}

#[derive(Default)]
pub struct Grid(HashMap<Point, Occupied>);

impl Grid {
    fn insert(&mut self, point: Point, occupied: Occupied) {
        let entry = self.0.entry(point).or_insert(occupied);
        if occupied > *entry {
            *entry = occupied;
        }
    }
}

pub struct GridRow {
    row: i32,
    items: HashMap<i32, Occupied>,
}

impl GridRow {
    pub fn new(row: i32) -> Self {
        GridRow {
            row,
            items: Default::default(),
        }
    }

    fn insert(&mut self, point: Point, occupied: Occupied) {
        if point.y != self.row {
            return;
        }

        let entry = self.items.entry(point.x).or_insert(occupied);
        if occupied > *entry {
            *entry = occupied;
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Occupied {
    /// The point is known to have neither because of the area a sensor covers
    Covered,
    /// The point has a beacon in it
    Beacon,
    /// The point has a sensor in it
    Sensor,
}

struct CoveredAreaIterator {
    sensor: Sensor,
    distance: i32,
    next_point: Option<Point>,
}

impl CoveredAreaIterator {
    fn new(sensor: Sensor, distance: i32) -> Self {
        let next_point = if distance < 0 {
            None
        } else {
            Some(sensor.point().up(distance))
        };
        CoveredAreaIterator {
            sensor,
            distance,
            next_point,
        }
    }
}

impl Iterator for CoveredAreaIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next_point?;
        let mut next_point = result.right(1);
        if self.sensor.point().manhatten_distance_to(next_point) > self.distance {
            next_point.y += 1;
            next_point.x = self.sensor.point().x;
            let distance = self.sensor.point().manhatten_distance_to(next_point);
            if distance > self.distance {
                self.next_point = None;
            } else {
                next_point.x -= self.distance - distance;
                self.next_point = Some(next_point);
            }
        } else {
            self.next_point = Some(next_point);
        }

        Some(result)
    }
}

struct CoveredAreaOnRowIterator {
    sensor: Sensor,
    distance: i32,
    next_point: Option<Point>,
}

impl CoveredAreaOnRowIterator {
    fn new(sensor: Sensor, distance: i32, row: i32) -> Self {
        let mut next_point = sensor.point();
        next_point.y = row;
        let distance_to_point = sensor.point().manhatten_distance_to(next_point);
        let next_point = if distance_to_point > distance {
            None
        } else {
            let diff = distance - distance_to_point;
            Some(next_point.left(diff))
        };
        CoveredAreaOnRowIterator {
            sensor,
            distance,
            next_point,
        }
    }
}

impl Iterator for CoveredAreaOnRowIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next_point?;
        let next_point = result.right(1);
        if self.sensor.point().manhatten_distance_to(next_point) > self.distance {
            self.next_point = None;
        } else {
            self.next_point = Some(next_point);
        }

        Some(result)
    }
}

struct Pair {
    sensor: Sensor,
    beacon: Beacon,
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() != 10 {
            return Err("Wrong number of parts");
        }

        let sensor_x = parts[2];
        let sensor_y = parts[3];
        let beacon_x = parts[8];
        let beacon_y = parts[9];

        let sensor_x = sensor_x
            .strip_prefix("x=")
            .and_then(|v| v.strip_suffix(','))
            .and_then(|v| v.parse().ok())
            .ok_or("sensor x invalid")?;
        let sensor_y = sensor_y
            .strip_prefix("y=")
            .and_then(|v| v.strip_suffix(':'))
            .and_then(|v| v.parse().ok())
            .ok_or("sensor y invalid")?;
        let beacon_x = beacon_x
            .strip_prefix("x=")
            .and_then(|v| v.strip_suffix(','))
            .and_then(|v| v.parse().ok())
            .ok_or("beacon x invalid")?;
        let beacon_y = beacon_y
            .strip_prefix("y=")
            .and_then(|v| v.parse().ok())
            .ok_or("beacon y invalid")?;

        let sensor = Point {
            x: sensor_x,
            y: sensor_y,
        }
        .into();
        let beacon = Point {
            x: beacon_x,
            y: beacon_y,
        }
        .into();
        Ok(Pair { sensor, beacon })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_covered_area() {
        let sensor = Sensor::from(Point { x: 0, y: 0 });
        let beacon = Beacon::from(Point { x: 0, y: -1 });
        let covered_area = sensor.covered_area(beacon).collect::<HashSet<_>>();
        assert!(covered_area.contains(&Point { x: 0, y: 0 }));
        assert!(covered_area.contains(&Point { x: 0, y: -1 }));
        assert!(covered_area.contains(&Point { x: 0, y: 1 }));
        assert!(covered_area.contains(&Point { x: -1, y: 0 }));
        assert!(covered_area.contains(&Point { x: 1, y: 0 }));
        assert!(!covered_area.contains(&Point { x: 1, y: 1 }));

        // .........
        // ....#....
        // ...B##...
        // ..#####..
        // .###S###.
        // ..#####..
        // ...###...
        // ....#....
        // .........
        let mut count = 0;
        let beacon = Beacon::from(Point { x: -1, y: -2 });
        for point in sensor.covered_area(beacon) {
            assert!(sensor.point().manhatten_distance_to(point) <= 3);
            count += 1;
        }
        assert_eq!(count, 1 + 3 + 5 + 7 + 5 + 3 + 1);
    }

    #[test]
    fn test_covered_area_on_row() {
        let sensor = Sensor::from(Point { x: 0, y: 0 });

        // .........
        // ....#....
        // ...B##...
        // ..#####..
        // .###S###.
        // ..#####.. <--
        // ...###...
        // ....#....
        // .........
        let mut count = 0;
        let beacon = Beacon::from(Point { x: -1, y: -2 });
        for point in sensor.covered_area_on_row(beacon, 1) {
            assert!(sensor.point().manhatten_distance_to(point) <= 3);
            count += 1;
        }
        assert_eq!(count, 5);

        let beacon = Beacon::from(Point { x: -7, y: -3 });
        let covered_area_result = sensor
            .covered_area(beacon)
            .filter(|pt| pt.y == 1)
            .collect::<HashSet<_>>();
        let covered_area_on_row_result = sensor
            .covered_area_on_row(beacon, 1)
            .collect::<HashSet<_>>();
        assert_eq!(covered_area_result, covered_area_on_row_result);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let pairs = input
            .trim()
            .lines()
            .map(|line| line.parse::<Pair>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part_1(&pairs, 10);
        assert_eq!(result, 26);
    }
}
