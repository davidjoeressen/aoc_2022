use std::io;

fn run(file: String) -> io::Result<()> {
    let fs = day_07::parse(&file);
    println!("Part 1: {}", day_07::part1(&fs));
    println!("Part 2: {}", day_07::part2(&fs));
    Ok(())
}

fn main() {
    util::execute(run);
}
