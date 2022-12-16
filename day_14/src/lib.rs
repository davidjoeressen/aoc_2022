use std::ops::{Index, IndexMut};
use std::str::FromStr;
use util::vec2d::{Offset, Point, Vec2d};

#[derive(PartialEq, Eq, Debug)]
pub struct Line(Vec<Point>);
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CaveMap {
    offset: Offset,
    data: Vec2d<bool>,
}

impl CaveMap {
    fn print(&self) {
        let height = self.data.height;
        let width = self.data.width;
        for y in 0..height {
            let start = width * y;
            let line: String = self.data.data[start..start + width]
                .iter()
                .map(|&x| if x { '#' } else { '.' })
                .collect();
            println!("{}", line);
        }
    }

    fn get(&self, index: Point) -> Option<bool> {
        if index.0 < self.offset.0 as usize || index.1 < self.offset.1 as usize {
            return None;
        }
        self.data.get(index - self.offset)
    }

    fn draw_section(&mut self, from: Point, to: Point) {
        if from.0 == to.0 {
            let a = from.1.min(to.1);
            let b = from.1.max(to.1);
            for y in a..=b {
                self[Point(from.0, y)] = true;
            }
        } else if from.1 == to.1 {
            let a = from.0.min(to.0);
            let b = from.0.max(to.0);
            for x in a..=b {
                self[Point(x, from.1)] = true;
            }
        } else {
            panic!("Cannot draw diagonal lines");
        }
    }

    fn draw_line(&mut self, line: &Line) {
        let mut iter = line.0.iter();
        let mut last = *iter.next().unwrap();
        while let Some(&next) = iter.next() {
            self.draw_section(last, next);
            last = next;
        }
    }
}

impl Index<Point> for CaveMap {
    type Output = bool;
    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index - self.offset]
    }
}
impl IndexMut<Point> for CaveMap {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index - self.offset]
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
    let mut min_x = i32::max_value();
    let mut max_x = 0;
    let mut max_y = 0;
    for Line(line) in lines.iter() {
        for Point(x, y) in line.iter() {
            let x = *x as i32;
            let y = *y as i32;
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }
    let width = max_x - min_x + 1;
    let height = max_y + 1;
    let mut map = CaveMap {
        offset: Offset(min_x, 0),
        data: Vec2d::new(width as usize, height as usize, false),
    };
    for line in lines.iter() {
        map.draw_line(line);
    }
    map
}

pub fn part1(mut map: CaveMap) -> u32 {
    let mut count = 0u32;
    let mut trail = Vec::new();
    let mut check = Point(500, 0);
    let offsets = vec![Offset(0, 1), Offset(-1, 1), Offset(1, 1)];
    'outer: loop {
        println!("Count: {}, Trail length: {}", count, trail.len());
        println!("Checking {:?}", check);
        for &offset in offsets.iter() {
            match map.get(check + offset) {
                Some(true) => {
                    println!("{:?} is blocked", check + offset);
                    continue;
                }
                Some(false) => {
                    println!("{:?} is free", check + offset);
                    trail.push(check);
                    check += offset;
                    continue 'outer;
                }
                None => {
                    return count;
                }
            }
        }
        map[check] = true;
        check = trail.pop().expect("This should never happen");
        count += 1;
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
    fn can_create_map() {
        let have = "498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let map = parse(have);
        assert_eq!(map.offset, Offset(494, 0));
        assert_eq!(map.data.width, 10);
        assert_eq!(map.data.height, 10);
    }

    #[test]
    fn can_set_points_on_map() {
        let mut map = CaveMap {
            offset: Offset(494, 0),
            data: Vec2d::new(10, 10, false),
        };
        let index = Point(498, 5);
        assert_eq!(map[index], false);
        map[index] = true;
        assert_eq!(map[index], true);
        assert_eq!(map.get(index), Some(true));
    }

    #[test]
    fn test_part1() {
        let have = "498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let map = parse(have);
        map.print();
        assert_eq!(part1(map), 24);
    }
}
