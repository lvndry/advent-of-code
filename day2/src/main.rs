use std::fs;

fn main() {
    let input: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    println!("Final vector {:?}", get_opcode(input, 12, 2));
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
