fn run(file: &str) {
    let file = file.trim();
    println!("Part 1: {}", day_06::part1(file));
    println!("Part 2: {}", day_06::part2(file));
}

fn main() {
    util::execute(run);
}
