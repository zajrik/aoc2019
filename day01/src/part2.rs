pub fn part2() {
    let input: Vec<u32> = include_str!("./input.txt")
        .replace("\r\n", "\n")
        .split("\n")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut fuel_reqs: Vec<u32> = Vec::new();

    for n in input {
        let mut fuel_fuel_reqs: Vec<u32> = Vec::new();
        fuel_fuel_reqs.push((n / 3) - 2);

        loop {
            let prev_fuel: u32 = fuel_fuel_reqs[fuel_fuel_reqs.len() - 1];
            let req: i64 = (prev_fuel as i64 / 3) - 2;

            if req <= 0 {
                break;
            }

            fuel_fuel_reqs.push(req as u32);
        }

        fuel_reqs.push(fuel_fuel_reqs.iter().fold(0, |a, b| a + b));
    }

    println!("{}", fuel_reqs.iter().fold(0, |a, b| a + b));
}
