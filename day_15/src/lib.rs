use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Range(i32, i32);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point(i32, i32);
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Sensor {
    position: Point,
    beacon: Point,
    distance: i32,
}

impl Point {
    fn distance(self, rhs: Point) -> i32 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }
}

impl Sensor {
    fn new(position: Point, beacon: Point) -> Sensor {
        let distance = position.distance(beacon);
        Sensor {
            position,
            beacon,
            distance,
        }
    }
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").ok_or("String doesn't contain a comma")?;
        let x = x
            .strip_prefix("x=")
            .ok_or("x coordinate not starting with x=")
            .and_then(|s| s.parse().map_err(|_| "Error parsing x coordinate"))?;
        let y = y
            .strip_prefix("y=")
            .ok_or("y coordinate not starting with y=")
            .and_then(|s| s.parse().map_err(|_| "Error parsing y coordinate"))?;
        Ok(Point(x, y))
    }
}

impl FromStr for Sensor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, beacon) = s
            .strip_prefix("Sensor at ")
            .and_then(|s| s.split_once(": closest beacon is at "))
            .ok_or("error splitting input")?;
        Ok(Sensor::new(position.parse()?, beacon.parse()?))
    }
}

pub fn parse(s: &str) -> Vec<Sensor> {
    s.lines().map(|s| s.parse().expect("Parse error")).collect()
}

fn get_ranges(sensors: &[Sensor], line: i32) -> Vec<Range> {
    sensors
        .iter()
        .filter_map(|sensor| {
            let d = sensor.distance - (sensor.position.1 - line).abs();
            let x = sensor.position.0;
            if d >= 0 {
                Some(Range(x - d, x + d))
            } else {
                None
            }
        })
        .collect()
}

fn count_with_overlap(mut ranges: Vec<Range>) -> i32 {
    ranges.sort();
    let mut check = i32::min_value();
    let mut count = 0;
    for Range(start, end) in ranges {
        let start = start.max(check);
        if end >= check {
            count += end - start + 1;
            check = end + 1;
        }
    }
    count
}

fn blocked_fields(sensors: &[Sensor], line: i32) -> i32 {
    let ranges: Vec<Range> = get_ranges(sensors, line);
    let blocked = count_with_overlap(ranges);
    let beacons: HashSet<i32> = sensors
        .iter()
        .filter(|sensor| sensor.beacon.1 == line)
        .map(|sensor| sensor.beacon.0)
        .collect();
    blocked - beacons.len() as i32
}

fn find_empty(sensors: &[Sensor], max: i32) -> i64 {
    for y in 0..=max {
        let mut x = 0;
        'row: while x <= max {
            let point = Point(x, y);
            for sensor in sensors {
                if sensor.position.distance(point) <= sensor.distance {
                    let d = sensor.distance - (sensor.position.1 - y).abs();
                    x = sensor.position.0 + d + 1;
                    continue 'row;
                }
            }
            return (x as i64) * 4_000_000 + (y as i64);
        }
    }
    -1
}

pub fn part1(sensors: &[Sensor]) -> i32 {
    blocked_fields(sensors, 2_000_000)
}

pub fn part2(sensors: &[Sensor]) -> i64 {
    find_empty(sensors, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_line() {
        let have = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let want = Sensor {
            position: Point(2, 18),
            beacon: Point(-2, 15),
            distance: 7,
        };
        assert_eq!(have.parse(), Ok(want));
    }

    #[test]
    fn test_part1_one_line() {
        let tests = vec![
            ("Sensor at x=2, y=18: closest beacon is at x=-2, y=15", 0),
            ("Sensor at x=9, y=16: closest beacon is at x=10, y=16", 0),
            ("Sensor at x=13, y=2: closest beacon is at x=15, y=3", 0),
            ("Sensor at x=12, y=14: closest beacon is at x=10, y=16", 1),
            ("Sensor at x=10, y=20: closest beacon is at x=10, y=16", 0),
            ("Sensor at x=14, y=17: closest beacon is at x=10, y=16", 0),
            ("Sensor at x=8, y=7: closest beacon is at x=2, y=10", 12),
            ("Sensor at x=2, y=0: closest beacon is at x=2, y=10", 0),
            ("Sensor at x=0, y=11: closest beacon is at x=2, y=10", 4),
            ("Sensor at x=20, y=14: closest beacon is at x=25, y=17", 9),
            ("Sensor at x=17, y=20: closest beacon is at x=21, y=22", 0),
            ("Sensor at x=16, y=7: closest beacon is at x=15, y=3", 5),
            ("Sensor at x=14, y=3: closest beacon is at x=15, y=3", 0),
            ("Sensor at x=20, y=1: closest beacon is at x=15, y=3", 0),
        ];
        for test in tests {
            assert_eq!(blocked_fields(&parse(test.0), 10), test.1);
        }
    }

    #[test]
    fn test_part1() {
        let have = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
            Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
            Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
            Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
            Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
            Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
            Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
            Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
            Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
            Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
            Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
            Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
            Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
            Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let data = parse(have);
        assert_eq!(blocked_fields(&data, 10), 26);
    }

    #[test]
    fn test_part2() {
        let have = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
            Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
            Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
            Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
            Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
            Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
            Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
            Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
            Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
            Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
            Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
            Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
            Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
            Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let data = parse(have);
        assert_eq!(find_empty(&data, 20), 56_000_011);
    }

    #[test]
    fn can_measure_ranges() {
        let tests = vec![
            (vec![Range(5, 10)], 6),
            (vec![Range(5, 10), Range(5, 7)], 6),
            (vec![Range(5, 10), Range(3, 12)], 10),
            (vec![Range(5, 10), Range(7, 12)], 8),
            (vec![Range(5, 10), Range(12, 12)], 7),
            (vec![Range(-2, 10)], 13),
        ];
        for (have, want) in tests {
            assert_eq!(count_with_overlap(have.clone()), want);
        }
    }
}
