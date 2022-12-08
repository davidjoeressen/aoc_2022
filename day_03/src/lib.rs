pub fn part1(s: &String) -> i64 {
    s.lines()
        .map(|l| find_overlap(l))
        .map(|c| priority(c))
        .sum()
}

pub fn part2(s: &String) -> i64 {
    let lines: Vec<&str> = s.lines().collect();
    lines
        .chunks(3)
        .map(|l| find_p2_overlap(l[0], l[1], l[2]))
        .map(|c| priority(c))
        .sum()
}

fn find_p2_overlap(x: &str, y: &str, z: &str) -> char {
    let x: Vec<char> = x.chars().collect();
    let y: Vec<char> = y.chars().collect();
    let z: Vec<char> = z.chars().collect();

    *find_overlaps(find_overlaps(x, y), z).iter().nth(0).unwrap()
}

fn find_overlap(s: &str) -> char {
    let chars: Vec<char> = s.chars().collect();
    let mid = chars.len() / 2;
    let (front, back) = chars.split_at(mid);
    *find_overlaps(front.to_vec(), back.to_vec())
        .iter()
        .nth(0)
        .unwrap()
}

fn find_overlaps<T: PartialEq + Copy>(x: Vec<T>, y: Vec<T>) -> Vec<T> {
    x.iter().filter(|a| y.contains(a)).map(|x| *x).collect()
}

fn priority(c: char) -> i64 {
    if ('a'..='z').contains(&c) {
        c as i64 - 'a' as i64 + 1
    } else if ('A'..='Z').contains(&c) {
        c as i64 - 'A' as i64 + 27
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priorities() {
        for (c, p) in ('a'..='z').zip(1..) {
            assert_eq!(priority(c), p);
        }
        for (c, p) in ('A'..='Z').zip(27..) {
            assert_eq!(priority(c), p);
        }
    }

    #[test]
    fn overlaps() {
        let tests = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
            ("PmmdzqPrVvPwwTWBwg", 'P'),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
            ("ttgJtRGJQctTZtZT", 't'),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
        ];
        for (bag, overlap) in tests {
            assert_eq!(find_overlap(bag), overlap);
        }
    }
}
