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

pub fn part1() -> Result<i32, std::io::Error> {
    let mut input: Vec<i32> = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut ptr: usize = 0;
    let mut out: i32 = 0;

    loop {
        let instruction: Instruction = Instruction::from(input[ptr]);

        match instruction.op {
            // Add
            1 => {
                let val1: i32 = Instruction::get_value(&input, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&input, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&input, ptr + 3, Mode::Immediate);
                input[idx as usize] = val1 + val2;
                ptr += 4;
            }

            // Multiply
            2 => {
                let val1: i32 = Instruction::get_value(&input, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&input, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&input, ptr + 3, Mode::Immediate);
                input[idx as usize] = val1 * val2;
                ptr += 4;
            }

            // Write input to memory
            3 => {
                let idx: i32 = Instruction::get_value(&input, ptr + 1, Mode::Immediate);

                // Simulate input of 1
                input[idx as usize] = 1;

                ptr += 2;
            }

            // Output from memory
            4 => {
                let val: i32 = Instruction::get_value(&input, ptr + 1, instruction.p1);
                out = val;
                ptr += 2;
            }

            // Exit
            99 => break,

            _ => panic!("Invalid opcode"),
        }
    }

    Ok(out)
}
