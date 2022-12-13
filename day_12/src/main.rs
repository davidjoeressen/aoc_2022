fn run(file: &str) {
    let map = file.parse().unwrap();
    let distance_map = day_12::generate_distance_map(&map);
    println!("Part 1: {}", day_12::part1(&map, &distance_map));
    println!("Part 2: {}", day_12::part2(&map, &distance_map));
}

fn main() {
    util::execute(run);
}
