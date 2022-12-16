use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point(pub usize, pub usize);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Offset(pub i32, pub i32);

fn add(x: usize, y: i32) -> usize {
    if y < 0 {
        x - y.abs() as usize
    } else {
        x + y as usize
    }
}

fn sub(x: usize, y: i32) -> usize {
    if y < 0 {
        x + y.abs() as usize
    } else {
        x - y as usize
    }
}

impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, rhs: Offset) -> Self::Output {
        Point(add(self.0, rhs.0), add(self.1, rhs.1))
    }
}

impl AddAssign<Offset> for Point {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Sub<Offset> for Point {
    type Output = Point;
    fn sub(self, rhs: Offset) -> Self::Output {
        Point(sub(self.0, rhs.0), sub(self.1, rhs.1))
    }
}

impl SubAssign<Offset> for Point {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("Parse error: No , found")?;
        Ok(Point(
            x.parse().or(Err(format!("Error parsing int {}", x)))?,
            y.parse().or(Err(format!("Error parsing int {}", y)))?,
        ))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Vec2d<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Vec2d<T> {
    pub fn new(width: usize, height: usize, init: T) -> Vec2d<T> {
        Vec2d {
            width,
            height,
            data: vec![init.clone(); width * height],
        }
    }
}

impl<T: Copy> Vec2d<T> {
    pub fn get(&self, index: Point) -> Option<T> {
        if index.0 < self.width && index.1 < self.height {
            Some(self[index])
        } else {
            None
        }
    }
}

impl<T> Index<Point> for Vec2d<T> {
    type Output = T;
    fn index(&self, index: Point) -> &Self::Output {
        let i = index.0 + index.1 * self.width;
        &self.data[i]
    }
}

impl<T> IndexMut<Point> for Vec2d<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let i = index.0 + index.1 * self.width;
        &mut self.data[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_offset_to_point() {
        let point = Point(5, 5);
        let tests = vec![
            (Offset(1, 1), Point(6, 6)),
            (Offset(-1, -1), Point(4, 4)),
            (Offset(0, 1), Point(5, 6)),
            (Offset(-1, 1), Point(4, 6)),
        ];
        for (offset, want) in tests {
            assert_eq!(point + offset, want);
        }
    }

    #[test]
    fn can_subtract_offset_to_point() {
        let point = Point(5, 5);
        let tests = vec![
            (Offset(1, 1), Point(4, 4)),
            (Offset(-1, -1), Point(6, 6)),
            (Offset(0, 1), Point(5, 4)),
            (Offset(-1, 1), Point(6, 4)),
        ];
        for (offset, want) in tests {
            assert_eq!(point - offset, want);
        }
    }

    #[test]
    fn can_store_data() {
        let mut vec = Vec2d::new(10, 10, false);
        let index = Point(5, 5);
        vec[index] = true;
        assert_eq!(vec[index], true);
        assert_eq!(vec.get(index), Some(true));
    }
}
