use std::io;

fn run(_file: String) -> io::Result<()> {
    // let monkeys = day_11::parse(&file);
    println!("Part 1: {}", day_11::part1(day_11::get_monkeys()));
    println!("Part 2: {}", day_11::part2(day_11::get_monkeys()));
    Ok(())
}

fn main() {
    util::execute(run);
}
