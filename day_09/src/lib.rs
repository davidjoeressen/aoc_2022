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
    tails: Vec<(i32, i32)>,
}

impl Rope {
    fn new(num_tails: usize) -> Rope {
        Rope {
            head: (0, 0),
            tails: vec![(0, 0); num_tails],
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
        let mut previous = self.head;
        for tail in self.tails.iter_mut() {
            let x = previous.0 - tail.0;
            let y = previous.1 - tail.1;
            if x == x.signum() && y == y.signum() {
                previous = *tail;
                continue;
            }
            tail.0 += x.signum();
            tail.1 += y.signum();
            previous = *tail;
        }
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
    let mut rope = Rope::new(1);
    let mut tail_positions = HashSet::new();
    for (dir, n) in instructions {
        for _ in 0..*n {
            rope.move_one(dir);
            tail_positions.insert(*rope.tails.last().unwrap());
        }
    }
    tail_positions.len() as i32
}

pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut rope = Rope::new(9);
    let mut tail_positions = HashSet::new();
    for (dir, n) in instructions {
        for _ in 0..*n {
            rope.move_one(dir);
            tail_positions.insert(*rope.tails.last().unwrap());
        }
    }
    tail_positions.len() as i32
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

    #[test]
    fn test_part2() {
        let have = vec![
            (Direction::Right, 5),
            (Direction::Up, 8),
            (Direction::Left, 8),
            (Direction::Down, 3),
            (Direction::Right, 17),
            (Direction::Down, 10),
            (Direction::Left, 25),
            (Direction::Up, 20),
        ];
        assert_eq!(part2(&have), 36);
    }
}
