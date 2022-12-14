fn run(file: &str) {
    let pairs = day_13::parse(file);
    println!("Part 1: {}", day_13::part1(&pairs));
    println!("Part 2: {}", day_13::part2(pairs));
}

fn main() {
    util::execute(run);
}
