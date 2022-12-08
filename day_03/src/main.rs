use day_03::{part1, part2};
use std::io;

fn run(file: String) -> io::Result<()> {
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
    Ok(())
}

fn main() {
    util::execute(run);
}
