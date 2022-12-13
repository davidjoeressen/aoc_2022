fn run(file: &str) {
    let fs = day_07::parse(file);
    println!("Part 1: {}", day_07::part1(&fs));
    println!("Part 2: {}", day_07::part2(&fs));
}

fn main() {
    util::execute(run);
}
