fn run(file: &str) {
    let map = day_14::parse(file);
    println!("Part 1: {}", day_14::part1(map.clone()));
}

fn main() {
    util::execute(run);
}
