use std::collections::BTreeSet;
use std::iter::Cycle;
use std::ops::{Add, AddAssign};
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
}

pub fn parse(s: &str) -> Vec<Direction> {
    s.trim()
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Parse error"),
        })
        .collect()
}

pub fn part1(directions: &[Direction]) -> i64 {
    let shapes = get_shapes();
    let mut game = Game::new(&shapes, directions);
    for _ in 0..2022 {
        game.drop();
    }
    game.height()
}

pub fn part2(directions: &[Direction]) -> i64 {
    let shapes = get_shapes();
    let mut game = Game::new(&shapes, directions);
    // for _ in 0..50_000 {
    //     game.drop();
    // }
    game.height()
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point(i64, i64);

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Piece(Vec<Point>);

impl Add<Point> for Piece {
    type Output = Self;
    fn add(mut self, rhs: Point) -> Self {
        self.0.iter_mut().for_each(|p| *p += rhs);
        self
    }
}

impl AddAssign<Point> for Piece {
    fn add_assign(&mut self, rhs: Point) {
        self.0.iter_mut().for_each(|p| *p += rhs);
    }
}

struct Game<'a> {
    width: i64,
    points: BTreeSet<Point>,
    shapes: Cycle<Iter<'a, Piece>>,
    directions: Cycle<Iter<'a, Direction>>,
}

impl<'a> Game<'a> {
    fn new(shapes: &'a [Piece], directions: &'a [Direction]) -> Self {
        Self {
            width: 7,
            points: BTreeSet::new(),
            shapes: shapes.iter().cycle(),
            directions: directions.iter().cycle(),
        }
    }

    fn height(&self) -> i64 {
        if let Some(point) = self.points.last() {
            point.0 + 1
        } else {
            0
        }
    }

    fn drop(&mut self) {
        let mut piece = self
            .shapes
            .next()
            .expect("there should always be a next piece")
            .clone()
            + Point(self.height() + 3, 2);
        loop {
            let h_offset = match *self
                .directions
                .next()
                .expect("there should always be a horizontal direction")
            {
                Direction::Left => Point(0, -1),
                Direction::Right => Point(0, 1),
            };
            if let Some(p) = self.try_move(&piece, h_offset) {
                piece = p;
            }
            if let Some(p) = self.try_move(&piece, Point(-1, 0)) {
                piece = p;
            } else {
                break;
            }
        }
        for &point in piece.0.iter() {
            self.points.insert(point);
        }
    }

    fn try_move(&self, piece: &Piece, offset: Point) -> Option<Piece> {
        let piece: Piece = piece.clone() + offset;
        if piece.0.iter().all(|point| {
            point.1 >= 0 && point.1 < self.width && point.0 >= 0 && !self.points.contains(point)
        }) {
            Some(piece)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let height = self.height();
        for line in (0.min(height - 16)..height).rev() {
            print!("|");
            for y in 0..self.width {
                if self.points.contains(&Point(line, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        if height < 16 {
            println!("+-------+");
        }
    }
}

fn get_shapes() -> Vec<Piece> {
    vec![
        // ####
        Piece(vec![Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)]),
        // .#.
        // ###
        // .#.
        Piece(vec![
            Point(0, 1),
            Point(1, 0),
            Point(1, 1),
            Point(1, 2),
            Point(2, 1),
        ]),
        // ..#
        // ..#
        // ###
        Piece(vec![
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(1, 2),
            Point(2, 2),
        ]),
        // #
        // #
        // #
        // #
        Piece(vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)]),
        // ##
        // ##
        Piece(vec![Point(0, 0), Point(0, 1), Point(1, 0), Point(1, 1)]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        use Direction::*;
        assert_eq!(
            parse(">><>><>"),
            vec![Right, Right, Left, Right, Right, Left, Right]
        );
    }

    #[test]
    fn solve_part1() {
        let data = parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part1(&data), 3068);
    }

    #[test]
    fn solve_part2() {
        let data = parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part2(&data), 1_514_285_714_288);
    }
}
