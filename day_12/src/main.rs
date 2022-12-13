fn run(file: &str) {
    let map = file.parse().unwrap();
    println!("Part 1: {}", day_12::part1(&map));
    println!("Part 2: {}", day_12::part2(&map));
}

fn main() {
    util::execute(run);
}
