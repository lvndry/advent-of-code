use std::fs;

fn main() {
    let input: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

fn part1(input: Vec<usize>) -> usize {
    get_opcode(input, 12, 2)
}

fn part2(input: Vec<usize>) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            if get_opcode(input.clone(), noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

fn get_opcode(mut input: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut index = 0;
    input[1] = noun;
    input[2] = verb;

    loop {
        match input[index] {
            1 => {
                let target = input[index + 3];
                input[target] = input[input[index + 1]] + input[input[index + 2]];
            }
            2 => {
                let target = input[index + 3];
                input[target] = input[input[index + 1]] * input[input[index + 2]];
            }
            99 => break,
            _ => panic!("Unexpected operator"),
        }
        index += 4;
    }

    input[0]
}
