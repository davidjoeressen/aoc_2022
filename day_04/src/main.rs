use day_04::{parse_file, part1, part2};
use std::io;

fn run(file: String) -> io::Result<()> {
    let pairs = parse_file(file);
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
    Ok(())
}

fn main() {
    util::execute(run);
}
