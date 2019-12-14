use std::iter::successors;

pub fn part1(input: &Vec<u32>) -> u32 {
    input.iter().map(|x| x / 3).map(|x| x - 2).sum()
}

pub fn part2(masses: &Vec<u32>) -> u32 {
    let fuel = |mass: &u32| (mass / 3).checked_sub(2);
    masses
        .iter()
        .map(|&mass| successors(fuel(&mass), |m| fuel(m)).sum::<u32>())
        .sum()
}
