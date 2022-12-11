mod map;

use map::Map;

pub fn parse(s: &str) -> Map<char> {
    let lines: Vec<&str> = s.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let values = lines
        .iter()
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    Map {
        width,
        height,
        values,
    }
}

pub fn part1(map: &Map<char>) -> i64 {
    let mut trees = Map {
        width: map.width,
        height: map.height,
        values: vec![false; map.values.len()],
    };
    let mut cur;
    for row in 0..map.height {
        cur = '/';
        for (col, c) in map.row(row).enumerate() {
            if c > cur {
                trees.set(row, col, true);
                cur = c;
            }
        }
        cur = '/';
        for (col, c) in map.row(row).enumerate().rev() {
            if c > cur {
                trees.set(row, col, true);
                cur = c;
            }
        }
    }
    for col in 0..map.width {
        cur = '/';
        for (row, c) in map.col(col).enumerate() {
            if c > cur {
                trees.set(row, col, true);
                cur = c;
            }
        }
        cur = '/';
        for (row, c) in map.col(col).enumerate().rev() {
            if c > cur {
                trees.set(row, col, true);
                cur = c;
            }
        }
    }
    trees.values.iter().filter(|&&x| x).count() as i64
}

fn scenic_score_at(map: &Map<char>, row: usize, col: usize) -> i64 {
    let height = map.get(row, col).unwrap();
    let mut result = 1i64;
    let mut sub_result = 0;
    // Check up
    for current_row in (0..row).rev() {
        sub_result += 1;
        if map.get(current_row, col).unwrap() >= height {
            break;
        }
    }
    result *= sub_result;
    sub_result = 0;
    // Check down
    for current_row in row + 1..map.height {
        sub_result += 1;
        if map.get(current_row, col).unwrap() >= height {
            break;
        }
    }
    result *= sub_result;
    sub_result = 0;
    // Check left
    for current_col in (0..col).rev() {
        sub_result += 1;
        if map.get(row, current_col).unwrap() >= height {
            break;
        }
    }
    result *= sub_result;
    sub_result = 0;
    // Check right
    for current_col in col + 1..map.width {
        sub_result += 1;
        if map.get(row, current_col).unwrap() >= height {
            break;
        }
    }
    result * sub_result
}

pub fn part2(map: &Map<char>) -> i64 {
    let mut max = 0i64;
    for row in 0..map.height {
        for col in 0..map.width {
            let scenic_score = scenic_score_at(map, row, col);
            if scenic_score > max {
                max = scenic_score;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let have = "30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390";
        let want = Map {
            width: 5,
            height: 5,
            values: "3037325512653323354935390".chars().collect(),
        };
        assert_eq!(parse(have), want);
    }

    #[test]
    fn test_part1() {
        let have = Map {
            width: 5,
            height: 5,
            values: "3037325512653323354935390".chars().collect(),
        };
        assert_eq!(part1(&have), 21);
    }

    #[test]
    fn test_part2() {
        let have = Map {
            width: 5,
            height: 5,
            values: "3037325512653323354935390".chars().collect(),
        };
        assert_eq!(part2(&have), 8);
    }

    #[test]
    fn test_scenic_score_at() {
        let have = Map {
            width: 5,
            height: 5,
            values: "3037325512653323354935390".chars().collect(),
        };
        assert_eq!(scenic_score_at(&have, 3, 2), 8);
    }
}
