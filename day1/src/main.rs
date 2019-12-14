use day1::{part1, part2};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut masses = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        masses.push(line.parse::<u32>().unwrap());
    }

    println!("Part 1: {}", part1(&masses));
    println!("Part 2: {}", part2(&masses));
    Ok(())
}
