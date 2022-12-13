fn run(file: &str) {
    let monkeys = day_11::parse(file);
    println!("Part 1: {}", day_11::part1(monkeys.clone()));
    println!("Part 2: {}", day_11::part2(monkeys));
}

fn main() {
    util::execute(run);
}
