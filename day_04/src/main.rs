use day_04::{parse_file, part1, part2};

fn run(file: &str) {
    let pairs = parse_file(file);
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
}

fn main() {
    util::execute(run);
}
