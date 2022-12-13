fn run(file: &str) {
    let instructions = day_09::parse(file);
    println!("Part 1: {}", day_09::part1(&instructions));
    println!("Part 2: {}", day_09::part2(&instructions));
}

fn main() {
    util::execute(run);
}
