fn run(file: &str) {
    let data = day_18::parse(file);
    println!("Part 1: {}", day_18::part1(&data));
    println!("Part 2: {}", day_18::part2(&data));
}

fn main() {
    util::execute(run)
}
