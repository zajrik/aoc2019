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
        let mut ints: Vec<u8> = instruction
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let op2: u8 = ints.pop().unwrap();
        let op1: u8 = match ints.pop() {
            Some(val) => val,
            None => 0,
        };

        let op: u8 = format!("{}{}", op1, op2).parse::<u8>().unwrap();
        let p1: Mode = Instruction::mode(ints.pop());
        let p2: Mode = Instruction::mode(ints.pop());

        Instruction { op, p1, p2 }
    }

    fn mode(int: Option<u8>) -> Mode {
        match int {
            Some(1) => Mode::Immediate,
            Some(0) => Mode::Position,
            None => Mode::Position,
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
            1 => {
                let val1: i32 = Instruction::get_value(&input, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&input, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&input, ptr + 3, Mode::Immediate);
                input[idx as usize] = val1 + val2;
                ptr += 4;
            }
            2 => {
                let val1: i32 = Instruction::get_value(&input, ptr + 1, instruction.p1);
                let val2: i32 = Instruction::get_value(&input, ptr + 2, instruction.p2);
                let idx: i32 = Instruction::get_value(&input, ptr + 3, Mode::Immediate);
                input[idx as usize] = val1 * val2;
                ptr += 4;
            }
            3 => {
                let idx: i32 = Instruction::get_value(&input, ptr + 1, Mode::Immediate);
                input[idx as usize] = 1;
                ptr += 2;
            }
            4 => {
                let val: i32 = Instruction::get_value(&input, ptr + 1, instruction.p1);
                out = val;
                ptr += 2;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }

    Ok(out)
}
