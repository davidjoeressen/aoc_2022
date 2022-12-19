fn run(file: &str) {
    let data = day_16::parse(file);
    println!("Part 1: {}", day_16::part1(&data));
    println!("Part 2: {}", day_16::part2(&data));
}

fn main() {
    util::execute(run);
}
