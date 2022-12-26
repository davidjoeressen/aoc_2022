fn run(file: &str) {
    let data = day_17::parse(file);
    println!("Part 1: {}", day_17::part1(&data));
    println!("Part 2: {}", day_17::part2(&data));
}

fn main() {
    util::execute(run);
}
