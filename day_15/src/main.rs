fn run(file: &str) {
    let sensors = day_15::parse(file);
    println!("Part 1: {}", day_15::part1(&sensors));
    println!("Part 2: {}", day_15::part2(&sensors));
}

fn main() {
    util::execute(run);
}
