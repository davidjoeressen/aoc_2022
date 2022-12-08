use day_02::{parse_file, part1, part2};
use std::io;

fn run(file: String) -> io::Result<()> {
    let game = parse_file(file);
    println!("Part 1: {}", part1(&game));
    println!("Part 2: {}", part2(&game));
    Ok(())
}

fn main() {
    util::execute(run);
}
