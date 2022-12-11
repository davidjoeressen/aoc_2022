use day_05::{parse, part1, part2};
use std::io;

fn run(file: String) -> io::Result<()> {
    let (crates, commands) = parse(&file);
    println!("Part 1: {}", part1(crates.clone(), &commands));
    println!("Part 2: {}", part2(crates, &commands));
    Ok(())
}

fn main() {
    util::execute(run);
}
