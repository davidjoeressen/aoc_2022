fn search_start(s: &str, marker_size: usize) -> usize {
    let mut tmp: Vec<char> = Vec::new();
    for (n, c) in s.chars().enumerate() {
        if let Some(index) = tmp.iter().rposition(|x| *x == c) {
            tmp = tmp.split_off(index + 1);
        }
        tmp.push(c);
        if tmp.len() == marker_size {
            return n + 1;
        }
    }
    return 0;
}

pub fn part1(s: &str) -> usize {
    search_start(s, 4)
}

pub fn part2(s: &str) -> usize {
    search_start(s, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let tests = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (have, want) in tests {
            assert_eq!(part1(have), want);
        }
    }

    #[test]
    fn test_part2() {
        let tests = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (have, want) in tests {
            assert_eq!(part2(have), want);
        }
    }
}
