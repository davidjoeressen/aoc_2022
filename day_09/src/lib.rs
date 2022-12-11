use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub type Instruction = (Direction, i32);

#[derive(Debug)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn move_one(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head.0 += 1,
            Direction::Down => self.head.0 -= 1,
            Direction::Left => self.head.1 -= 1,
            Direction::Right => self.head.1 += 1,
        };
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let x = self.head.0 - self.tail.0;
        let y = self.head.1 - self.tail.1;
        if x == x.signum() && y == y.signum() {
            return;
        }
        self.tail.0 += x.signum();
        self.tail.1 += y.signum();
    }
}

pub fn parse(file: &str) -> Vec<Instruction> {
    file.lines()
        .map(|line| {
            let tmp: Vec<&str> = line.split_whitespace().collect();
            let amount = str::parse(tmp[1]).unwrap();
            let dir = match tmp[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Parse error"),
            };
            (dir, amount)
        })
        .collect()
}

pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut rope = Rope::new();
    let mut tail_positions = HashSet::new();
    for (dir, n) in instructions {
        for _ in 0..*n {
            rope.move_one(dir);
            tail_positions.insert(rope.tail);
        }
    }
    tail_positions.len() as i32
}

pub fn part2(_instructions: &[Instruction]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "R 4\n\
            U 4\n\
            L 3\n\
            D 1\n\
            R 4\n\
            D 1\n\
            L 5\n\
            R 2";
        let want = vec![
            (Direction::Right, 4),
            (Direction::Up, 4),
            (Direction::Left, 3),
            (Direction::Down, 1),
            (Direction::Right, 4),
            (Direction::Down, 1),
            (Direction::Left, 5),
            (Direction::Right, 2),
        ];
        assert_eq!(parse(input), want);
    }

    #[test]
    fn test_part1() {
        let have = vec![
            (Direction::Right, 4),
            (Direction::Up, 4),
            (Direction::Left, 3),
            (Direction::Down, 1),
            (Direction::Right, 4),
            (Direction::Down, 1),
            (Direction::Left, 5),
            (Direction::Right, 2),
        ];
        assert_eq!(part1(&have), 13);
    }
}
