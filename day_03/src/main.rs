use day_03::{part1, part2};

fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}

fn main() {
    util::execute(run);
}
