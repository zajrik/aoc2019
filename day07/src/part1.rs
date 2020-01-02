use std::slice::Iter;

enum Mode {
    Position,
    Immediate,
}

struct Instruction {
    op: u8,
    p1: Mode,
    p2: Mode,
}

impl Instruction {
    fn from(instruction: i32) -> Instruction {
        let op: u8 = (instruction % 100) as u8;
        let p1: Mode = Instruction::mode((instruction / 100 % 10) as u8);
        let p2: Mode = Instruction::mode((instruction / 1000) as u8);

        Instruction { op, p1, p2 }
    }

    fn mode(int: u8) -> Mode {
        match int {
            1 => Mode::Immediate,
            0 => Mode::Position,
            _ => panic!("Invalid mode"),
        }
    }

    fn get_value(input: &Vec<i32>, ptr: usize, mode: Mode) -> i32 {
        match mode {
            Mode::Position => input[input[ptr] as usize],
            Mode::Immediate => input[ptr],
        }
    }
}

/// Execute the given program which will consume the given input
pub fn compute(program: &mut Vec<i32>, input: &Vec<i32>) -> i32 {
    let mut input_values: Iter<i32> = input.iter();
    let mut ptr: usize = 0;
    let mut out: i32 = 0;

    loop {
        let instruction: Instruction = Instruction::from(program[ptr]);

        match instruction.op {
            // Add
            1 => {
                let val1: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&program, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&program, ptr + 3, Mode::Immediate);
                program[idx as usize] = val1 + val2;
                ptr += 4;
            }

            // Multiply
            2 => {
                let val1: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&program, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&program, ptr + 3, Mode::Immediate);
                program[idx as usize] = val1 * val2;
                ptr += 4;
            }

            // Write input to memory
            3 => {
                let idx: i32 = Instruction::get_value(&program, ptr + 1, Mode::Immediate);
                program[idx as usize] = *input_values.next().unwrap() as i32;
                ptr += 2;
            }

            // Set output
            4 => {
                let val: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                out = val;
                ptr += 2;
            }

            // Jump if true (non-zero value)
            5 => {
                let val: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                let idx: i32 = Instruction::get_value(&program, ptr + 2, instruction.p2);

                if val > 0 {
                    ptr = idx as usize;
                } else {
                    ptr += 3;
                }
            }

            // Jump if false (zero value)
            6 => {
                let val: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                let idx: i32 = Instruction::get_value(&program, ptr + 2, instruction.p2);

                if val == 0 {
                    ptr = idx as usize;
                } else {
                    ptr += 3;
                }
            }

            // Less-than
            7 => {
                let val1: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&program, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&program, ptr + 3, Mode::Immediate);

                if val1 < val2 {
                    program[idx as usize] = 1;
                } else {
                    program[idx as usize] = 0;
                }

                ptr += 4;
            }

            // Equals
            8 => {
                let val1: i32 = Instruction::get_value(&program, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&program, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&program, ptr + 3, Mode::Immediate);

                if val1 == val2 {
                    program[idx as usize] = 1;
                } else {
                    program[idx as usize] = 0;
                }

                ptr += 4;
            }

            // Exit
            99 => break,

            _ => panic!("Invalid opcode"),
        }
    }

    out
}

/// Finds all permutations of the given set of values using Heap's algorithm
fn permutations<T: Copy>(input: Vec<T>) -> Vec<Vec<T>> {
    fn generate_permutations<T: Copy>(out: &mut Vec<Vec<T>>, input: &mut Vec<T>, len: usize) {
        if len <= 1 {
            out.push(input.clone());
            return;
        }

        for i in 0..len {
            generate_permutations(out, input, len - 1);

            if len & 1 == 0 {
                let first: T = input[0];
                input[0] = input[len - 1];
                input[len - 1] = first;
            } else {
                let first: T = input[i];
                input[i] = input[len - 1];
                input[len - 1] = first;
            }
        }
    }

    let len: usize = input.len();
    let mut out: Vec<Vec<T>> = Vec::new();
    generate_permutations(&mut out, &mut input.clone(), len);
    out
}

pub fn part1() -> Result<i32, std::io::Error> {
    let input: Vec<i32> = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let phase_setting_sets: Vec<Vec<i32>> = permutations(vec![0, 1, 2, 3, 4]);
    let mut largest_output: i32 = 0;

    for phase_settings in phase_setting_sets {
        let mut last_output: i32 = 0;

        for phase_setting in phase_settings {
            let result: i32 = compute(&mut input.clone(), &mut vec![phase_setting, last_output]);
            last_output = result;
        }

        if last_output > largest_output {
            largest_output = last_output;
        }
    }

    Ok(largest_output)
}
