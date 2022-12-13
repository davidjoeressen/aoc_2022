use std::io;

fn run(file: String) -> io::Result<()> {
    let monkeys = day_11::parse(&file);
    println!("Part 1: {}", day_11::part1(monkeys.clone()));
    println!("Part 2: {}", day_11::part2(monkeys));
    Ok(())
}

fn main() {
    util::execute(run);
}
