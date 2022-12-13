fn main() {
    util::execute(run)
}

fn run(content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let elves: Vec<i32> = lines
        .rsplit(|x| x.is_empty())
        .map(|x| x.iter().filter_map(|y| y.parse::<i32>().ok()).sum())
        .collect();

    let part1: i32 = *elves.iter().max().unwrap();
    println!("Part 1: {}", part1);

    let mut part2: [i32; 3] = [0; 3];
    for x in elves {
        let mut smallest_index = 0;
        let mut smallest_value = part2[0];
        for (n, item) in part2.iter().enumerate().skip(1) {
            if *item < smallest_value {
                smallest_index = n;
                smallest_value = *item;
            }
        }
        if smallest_value < x {
            part2[smallest_index] = x;
        }
    }
    println!("Part 2: {}", part2.iter().sum::<i32>());
}
