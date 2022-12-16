use std::collections::HashSet;
use std::str::FromStr;
use util::vec2d::{Offset, Point};

#[derive(PartialEq, Eq, Debug)]
pub struct Line(Vec<Point>);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CaveMap {
    floor: i32,
    data: HashSet<Point>,
}

impl CaveMap {
    fn new() -> Self {
        CaveMap {
            floor: 0,
            data: HashSet::new(),
        }
    }

    fn draw_section(&mut self, from: Point, to: Point) {
        if from.0 == to.0 {
            let a = from.1.min(to.1);
            let b = from.1.max(to.1);
            for y in a..=b {
                self.data.insert(Point(from.0, y));
            }
        } else if from.1 == to.1 {
            let a = from.0.min(to.0);
            let b = from.0.max(to.0);
            for x in a..=b {
                self.data.insert(Point(x, from.1));
            }
        } else {
            panic!("Cannot draw diagonal lines");
        }
    }

    fn draw_line(&mut self, line: &Line) {
        let mut iter = line.0.iter();
        let mut last = *iter.next().unwrap();
        for &next in iter {
            self.draw_section(last, next);
            last = next;
        }
    }
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(
            s.split(" -> ")
                .map(str::parse)
                .collect::<Result<Vec<Point>, String>>()?,
        ))
    }
}

pub fn parse(s: &str) -> CaveMap {
    let lines: Vec<Line> = s.lines().map(|s| s.parse().expect("Parse error")).collect();
    let mut map = CaveMap::new();
    for line in lines.iter() {
        map.draw_line(line);
    }
    map.floor = map
        .data
        .iter()
        .map(|point| point.1)
        .max()
        .expect("No points in map?!")
        + 2;
    map
}

pub fn part1(mut map: CaveMap) -> u32 {
    let mut count = 0u32;
    let mut trail = Vec::new();
    let mut check = Point(500, 0);
    let offsets = vec![Offset(0, 1), Offset(-1, 1), Offset(1, 1)];
    'outer: loop {
        for &offset in offsets.iter() {
            let field = check + offset;
            if field.1 > map.floor {
                return count;
            }
            if map.data.contains(&field) {
                continue;
            } else {
                trail.push(check);
                check += offset;
                continue 'outer;
            }
        }
        map.data.insert(check);
        check = trail.pop().expect("This should never happen");
        count += 1;
    }
}

pub fn part2(mut map: CaveMap) -> u32 {
    let mut count = 0u32;
    let mut trail = Vec::new();
    let mut check = Point(500, 0);
    let offsets = vec![Offset(0, 1), Offset(-1, 1), Offset(1, 1)];
    'outer: loop {
        for &offset in offsets.iter() {
            let field = check + offset;
            if field.1 >= map.floor || map.data.contains(&field) {
                continue;
            } else {
                trail.push(check);
                check += offset;
                continue 'outer;
            }
        }
        map.data.insert(check);
        count += 1;
        if let Some(next) = trail.pop() {
            check = next;
        } else {
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_line() {
        let have = "498,4 -> 498,6 -> 496,6";
        let want = Line(vec![Point(498, 4), Point(498, 6), Point(496, 6)]);
        assert_eq!(have.parse(), Ok(want));
    }

    #[test]
    fn test_part1() {
        let have = "498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let map = parse(have);
        assert_eq!(part1(map), 24);
    }

    #[test]
    fn test_part2() {
        let have = "498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let map = parse(have);
        assert_eq!(part2(map), 93);
    }
}
