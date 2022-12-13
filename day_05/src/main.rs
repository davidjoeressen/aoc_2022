use day_05::{parse, part1, part2};

fn run(file: &str) {
    let (crates, commands) = parse(file);
    println!("Part 1: {}", part1(crates.clone(), &commands));
    println!("Part 2: {}", part2(crates, &commands));
}

fn main() {
    util::execute(run);
}
