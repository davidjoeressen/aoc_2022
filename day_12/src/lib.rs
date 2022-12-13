use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

type Coordinate = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
pub struct Matrix<T> {
    height: usize,
    width: usize,
    values: Vec<T>,
}

impl<T> Matrix<T> {
    fn get_neighbors(&self, (x, y): Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x + 1 < self.height {
            neighbors.push((x + 1, y));
        }
        if y + 1 < self.width {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
}

impl<T: Clone> Matrix<T> {
    fn new(height: usize, width: usize, init: T) -> Matrix<T> {
        Matrix {
            height,
            width,
            values: vec![init; height * width],
        }
    }
}

impl Matrix<i32> {
    fn distance_map(&self, to: Coordinate) -> Matrix<Option<i32>> {
        let mut distances: Matrix<Option<i32>> = Matrix::new(self.height, self.width, None);
        distances[to] = Some(0);
        let mut queue = BinaryHeap::new();
        queue.push(Field(0, to));
        while let Some(Field(_, coord)) = queue.pop() {
            let distance = distances[coord].unwrap();
            let next_distance = distance + 1;
            let possible_height = self[coord] - 1;
            let neighbors = self.get_neighbors(coord);
            for &neighbor in neighbors.iter() {
                if self[neighbor] < possible_height {
                    continue;
                }
                if let Some(n) = distances[neighbor] {
                    if n <= next_distance {
                        continue;
                    }
                }
                distances[neighbor] = Some(next_distance);
                queue.push(Field(next_distance, neighbor));
            }
        }
        distances
    }
}

impl<T> Index<Coordinate> for Matrix<T> {
    type Output = T;
    fn index(&self, index: Coordinate) -> &Self::Output {
        let i = index.0 * self.width + index.1;
        &self.values[i]
    }
}

impl<T> IndexMut<Coordinate> for Matrix<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        let i = index.0 * self.width + index.1;
        &mut self.values[i]
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct HeightMap {
    start: Coordinate,
    goal: Coordinate,
    map: Matrix<i32>,
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let width = lines.peek().ok_or("Parse error")?.len();
        let height = lines.count();
        let chars: Vec<char> = s
            .lines()
            .flat_map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let start = chars
            .iter()
            .enumerate()
            .find(|(_, &c)| c == 'S')
            .ok_or("No start found")?
            .0;
        let start = (start / width, start % width);
        let goal = chars
            .iter()
            .enumerate()
            .find(|(_, &c)| c == 'E')
            .ok_or("No goal found")?
            .0;
        let goal = (goal / width, goal % width);
        let values = chars
            .iter()
            .map(|c| match c {
                'S' => Ok(0),
                'E' => Ok('z' as i32 - 'a' as i32),
                'a'..='z' => Ok(*c as i32 - 'a' as i32),
                _ => Err(format!("Illegal character: {}", c)),
            })
            .collect::<Result<Vec<i32>, Self::Err>>()?;
        Ok(HeightMap {
            start,
            goal,
            map: Matrix {
                height,
                width,
                values,
            },
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Field(i32, Coordinate);

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse().then(self.1.cmp(&other.1))
    }
}

pub fn generate_distance_map(map: &HeightMap) -> Matrix<Option<i32>> {
    map.map.distance_map(map.goal)
}

pub fn part1(map: &HeightMap, distance_map: &Matrix<Option<i32>>) -> i32 {
    distance_map[map.start].unwrap()
}

pub fn part2(map: &HeightMap, distance_map: &Matrix<Option<i32>>) -> i32 {
    let mut smallest_distance = i32::max_value();
    for x in 0..map.map.height {
        for y in 0..map.map.width {
            if map.map[(x, y)] != 0 {
                continue;
            }
            if let Some(n) = distance_map[(x, y)] {
                smallest_distance = smallest_distance.min(n);
            }
        }
    }
    smallest_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let have = "Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi";
        let want = HeightMap {
            start: (0, 0),
            goal: (2, 5),
            map: Matrix {
                height: 5,
                width: 8,
                values: vec![
                    0, 0, 1, 16, 15, 14, 13, 12, 0, 1, 2, 17, 24, 23, 23, 11, 0, 2, 2, 18, 25, 25,
                    23, 10, 0, 2, 2, 19, 20, 21, 22, 9, 0, 1, 3, 4, 5, 6, 7, 8,
                ],
            },
        };
        assert_eq!(have.parse(), Ok(want));
    }

    #[test]
    fn test_solution() {
        let have = "Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi";
        let map = have.parse().unwrap();
        let distance_map = generate_distance_map(&map);
        assert_eq!(part1(&map, &distance_map), 31);
        assert_eq!(part2(&map, &distance_map), 29);
    }
}
