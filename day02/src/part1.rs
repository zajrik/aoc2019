pub fn part1() {
    let mut input: Vec<u32> = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    input[1] = 12;
    input[2] = 2;

    let mut i: usize = 0;

    loop {
        match input[i] {
            1 => {
                let val1: u32 = input[input[i + 1] as usize];
                let val2: u32 = input[input[i + 2] as usize];
                let idx: usize = input[i + 3] as usize;
                input[idx] = val1 + val2;
            }
            2 => {
                let val1: u32 = input[input[i + 1] as usize];
                let val2: u32 = input[input[i + 2] as usize];
                let idx: usize = input[i + 3] as usize;
                input[idx] = val1 * val2;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }

        i += 4;
    }

    println!("{}", input[0]);
}
