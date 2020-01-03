use std::slice::Iter;

// Represents an intcode program instruction parameter mode
enum Mode {
    Position,
    Immediate,
}

// Represents an instruction in an intcode program
struct Instruction {
    op: u8,
    p1: Mode,
    p2: Mode,
}

impl Instruction {
    /// Parse an instruction from the given i32
    fn from(instruction: i32) -> Instruction {
        let op: u8 = (instruction % 100) as u8;
        let p1: Mode = Instruction::mode((instruction / 100 % 10) as u8);
        let p2: Mode = Instruction::mode((instruction / 1000) as u8);

        Instruction { op, p1, p2 }
    }

    /// Parse a parameter mode from the given u8
    fn mode(int: u8) -> Mode {
        match int {
            1 => Mode::Immediate,
            0 => Mode::Position,
            _ => panic!("Invalid mode"),
        }
    }
}

/// Represents an intcode program
struct Program<'a> {
    memory: &'a mut Vec<i32>,
    memptr: usize,
    input: Iter<'a, i32>,
    out: i32,
}

impl<'a> Program<'a> {
    /// Create a new Program from the given instruction vector and input vector
    fn new(program: &'a mut Vec<i32>, input: &'a Vec<i32>) -> Program<'a> {
        Program {
            memory: program,
            memptr: 0,
            input: input.iter(),
            out: 0,
        }
    }

    /// Get the next instruction
    fn get_instruction(&mut self) -> Instruction {
        let out: Instruction = Instruction::from(self.memory[self.memptr]);
        self.memptr += 1;
        out
    }

    /// Get next parameters using the given instruction modes
    fn get_parameters(&mut self, modes: Vec<Mode>) -> Vec<i32> {
        let out: Vec<i32> = modes
            .iter()
            .enumerate()
            .map(|(i, mode)| match mode {
                Mode::Position => self.memory[self.memory[self.memptr + i] as usize],
                Mode::Immediate => self.memory[self.memptr + i],
            })
            .collect();

        self.memptr += modes.len();
        out
    }

    /// Execute a program cycle. `Some(())` means the program exited
    fn do_cycle(&mut self) -> Option<()> {
        let instruction: Instruction = self.get_instruction();

        match instruction.op {
            // Add
            1 => {
                let modes: Vec<Mode> = vec![instruction.p1, instruction.p2, Mode::Immediate];
                if let [val1, val2, idx] = self.get_parameters(modes)[..] {
                    self.memory[idx as usize] = val1 + val2;
                }
            }

            // Multiply
            2 => {
                let modes: Vec<Mode> = vec![instruction.p1, instruction.p2, Mode::Immediate];
                if let [val1, val2, idx] = self.get_parameters(modes)[..] {
                    self.memory[idx as usize] = val1 * val2;
                }
            }

            // Write input to memory
            3 => {
                if let [idx] = self.get_parameters(vec![Mode::Immediate])[..] {
                    self.memory[idx as usize] = *self.input.next()?;
                }
            }

            // Set output
            4 => {
                if let [val] = self.get_parameters(vec![instruction.p1])[..] {
                    self.out = val;
                }
            }

            // Jump if true (non-zero value)
            5 => {
                let modes: Vec<Mode> = vec![instruction.p1, instruction.p2];
                if let [val, idx] = self.get_parameters(modes)[..] {
                    if val > 0 {
                        self.memptr = idx as usize;
                    }
                }
            }

            // Jump if false (zero value)
            6 => {
                let modes: Vec<Mode> = vec![instruction.p1, instruction.p2];
                if let [val, idx] = self.get_parameters(modes)[..] {
                    if val == 0 {
                        self.memptr = idx as usize;
                    }
                }
            }

            // Less-than
            7 => {
                let modes: Vec<Mode> = vec![instruction.p1, instruction.p2, Mode::Immediate];
                if let [val1, val2, idx] = self.get_parameters(modes)[..] {
                    if val1 < val2 {
                        self.memory[idx as usize] = 1;
                    } else {
                        self.memory[idx as usize] = 0;
                    }
                }
            }

            // Equals
            8 => {
                let modes: Vec<Mode> = vec![instruction.p1, instruction.p2, Mode::Immediate];
                if let [val1, val2, idx] = self.get_parameters(modes)[..] {
                    if val1 == val2 {
                        self.memory[idx as usize] = 1;
                    } else {
                        self.memory[idx as usize] = 0;
                    }
                }
            }

            // Exit
            99 => return Some(()),

            _ => panic!("Invalid opcode"),
        }

        None
    }

    /// Run the program, returning the final output value
    fn run(&mut self) -> i32 {
        loop {
            match self.do_cycle() {
                Some(()) => return self.out,
                None => continue,
            }
        }
    }
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

    for phase_setting_set in phase_setting_sets {
        let mut last_output: i32 = 0;

        for phase_setting in phase_setting_set {
            let mut instructions: Vec<i32> = input.clone();
            let mut input_set: Vec<i32> = vec![phase_setting, last_output];
            last_output = Program::new(&mut instructions, &mut input_set).run();
        }

        if last_output > largest_output {
            largest_output = last_output;
        }
    }

    Ok(largest_output)
}
