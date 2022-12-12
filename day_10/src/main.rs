use std::io;

fn run(file: String) -> io::Result<()> {
    let commands = day_10::parse(&file);
    let values = day_10::calc_values(&commands);
    println!("Part 1: {}", day_10::part1(&values));
    println!("Part 2:");
    for line in day_10::part2(&values).iter() {
        println!("{}", line);
    }
    Ok(())
}

fn main() {
    util::execute(run);
}
