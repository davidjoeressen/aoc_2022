use std::str;

type Range = (i32, i32);

pub fn parse_file(file: &str) -> Vec<(Range, Range)> {
    file.lines().map(parse_line).collect()
}

pub fn part1(ranges: &[(Range, Range)]) -> usize {
    ranges
        .iter()
        .filter(|(x, y)| contains_fully(x, y) || contains_fully(y, x))
        .count()
}

pub fn part2(ranges: &[(Range, Range)]) -> usize {
    ranges.iter().filter(|(x, y)| overlaps(x, y)).count()
}

fn contains_fully((x1, x2): &Range, (y1, y2): &Range) -> bool {
    x1 <= y1 && x2 >= y2
}

fn overlaps(x @ (x1, x2): &Range, y @ (y1, y2): &Range) -> bool {
    in_range(x1, y) || in_range(x2, y) || in_range(y1, x) || in_range(y2, x)
}

fn in_range(x: &i32, (y1, y2): &Range) -> bool {
    *x >= *y1 && *x <= *y2
}

fn parse_range(range: &str) -> Range {
    let (x, y) = range.split_once('-').unwrap();
    (str::parse(x).unwrap(), str::parse(y).unwrap())
}

fn parse_line(line: &str) -> (Range, Range) {
    let (e1, e2) = line.split_once(',').unwrap();
    (parse_range(e1), parse_range(e2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parse() {
        assert_eq!(parse_line("30-31,2-31"), ((30, 31), (2, 31)));
    }

    #[test]
    fn file_parse() {
        let have = "2-4,6-8\n\
                    2-3,4-5\n\
                    5-7,7-9\n\
                    2-8,3-7\n\
                    6-6,4-6\n\
                    2-6,4-8";
        let want = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        assert_eq!(parse_file(have), want);
    }

    #[test]
    fn test_part1() {
        let have = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        assert_eq!(part1(&have), 2);
    }

    #[test]
    fn test_part2() {
        let have = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        assert_eq!(part2(&have), 4);
    }
}
