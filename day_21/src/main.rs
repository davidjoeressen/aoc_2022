fn run(file: &str) {
    let data = file.parse().unwrap();
    println!("Part 1: {}", day_21::part1(&data));
    println!("Part 2: {}", day_21::part2(data));
}

fn main() {
    util::execute(run);
}
