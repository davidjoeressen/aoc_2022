fn run(file: &str) {
    let data = day_08::parse(file);
    println!("Part 1: {}", day_08::part1(&data));
    println!("Part 2: {}", day_08::part2(&data));
}

fn main() {
    util::execute(run);
}
