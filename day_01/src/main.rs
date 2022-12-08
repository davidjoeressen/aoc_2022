use std::io;

fn main() {
    util::execute(run)
}

fn run(content: String) -> io::Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    let elves: Vec<i32> = lines
        .rsplit(|x| x.len() == 0)
        .map(|x| x.iter().filter_map(|y| y.parse::<i32>().ok()).sum())
        .collect();

    let part1: i32 = *elves.iter().max().unwrap();
    println!("Part 1: {}", part1);

    let mut part2: [i32; 3] = [0; 3];
    for x in elves {
        let mut smallest_index = 0;
        let mut smallest_value = part2[0];
        for n in 1..part2.len() {
            if part2[n] < smallest_value {
                smallest_index = n;
                smallest_value = part2[n];
            }
        }
        if smallest_value < x {
            part2[smallest_index] = x;
        }
    }
    println!("Part 2: {}", part2.iter().sum::<i32>());

    Ok(())
}
