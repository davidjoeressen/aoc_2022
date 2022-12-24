use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Point(i32, i32, i32);

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let offsets = vec![
            Point(1, 0, 0),
            Point(-1, 0, 0),
            Point(0, 1, 0),
            Point(0, -1, 0),
            Point(0, 0, 1),
            Point(0, 0, -1),
        ];
        offsets.iter().map(|offset| *self + *offset).collect()
    }
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        Ok(Point(
            iter.next()
                .ok_or("not enough values (got 0)")?
                .parse()
                .map_err(|_| "Error parsing first int")?,
            iter.next()
                .ok_or("not enough values (got 1)")?
                .parse()
                .map_err(|_| "Error parsing second int")?,
            iter.next()
                .ok_or("not enough values (got 2)")?
                .parse()
                .map_err(|_| "Error parsing third int")?,
        ))
    }
}

pub fn parse(s: &str) -> Vec<Point> {
    s.lines()
        .map(str::parse)
        .collect::<Result<Vec<Point>, String>>()
        .expect("Parse error")
}

pub fn part1(points: &[Point]) -> i32 {
    let hash: HashSet<Point> = points.iter().copied().collect();
    points
        .iter()
        .map(|point| {
            point
                .neighbours()
                .iter()
                .filter(|point| !hash.contains(point))
                .count() as i32
            //
        })
        .sum()
}

fn find_bounds(points: &HashSet<Point>) -> (Point, Point) {
    let mut lower = Point(i32::max_value(), i32::max_value(), i32::max_value());
    let mut upper = Point(i32::min_value(), i32::min_value(), i32::min_value());
    for point in points.iter() {
        lower.0 = lower.0.min(point.0);
        lower.1 = lower.1.min(point.1);
        lower.2 = lower.2.min(point.2);
        upper.0 = upper.0.max(point.0);
        upper.1 = upper.1.max(point.1);
        upper.2 = upper.2.max(point.2);
    }
    (lower + Point(-1, -1, -1), upper + Point(1, 1, 1))
}

fn is_oob(lower: &Point, upper: &Point, point: &Point) -> bool {
    !(lower.0..=upper.0).contains(&point.0)
        || !(lower.1..=upper.1).contains(&point.1)
        || !(lower.1..=upper.2).contains(&point.2)
}

fn find_outside_points(points: &HashSet<Point>) -> HashSet<Point> {
    let mut outside: HashSet<Point> = HashSet::new();
    let (lower_bound, upper_bound) = find_bounds(points);
    let mut queue: Vec<Point> = vec![lower_bound];
    while let Some(point) = queue.pop() {
        if outside.contains(&point) {
            continue;
        }
        outside.insert(point);
        for neighbor in point.neighbours().iter() {
            if outside.contains(neighbor)
                || points.contains(neighbor)
                || is_oob(&lower_bound, &upper_bound, neighbor)
            {
                continue;
            }
            queue.push(*neighbor);
        }
    }
    outside
}

pub fn part2(points: &[Point]) -> i32 {
    let hash: HashSet<Point> = points.iter().copied().collect();
    let outside = find_outside_points(&hash);
    points
        .iter()
        .map(|point| {
            point
                .neighbours()
                .iter()
                .filter(|point| !hash.contains(point))
                .filter(|point| outside.contains(point))
                .count() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_point() {
        let have = "25,10,13";
        assert_eq!(have.parse(), Ok(Point(25, 10, 13)));
    }

    #[test]
    fn solve() {
        let have = "\
            2,2,2\n\
            1,2,2\n\
            3,2,2\n\
            2,1,2\n\
            2,3,2\n\
            2,2,1\n\
            2,2,3\n\
            2,2,4\n\
            2,2,6\n\
            1,2,5\n\
            3,2,5\n\
            2,1,5\n\
            2,3,5";
        let data = parse(have);
        assert_eq!(part1(&data), 64);
        assert_eq!(part2(&data), 58);
    }
}
