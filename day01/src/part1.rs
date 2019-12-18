pub fn part1() {
    let input: Vec<u32> = include_str!("./input.txt")
        .replace("\r\n", "\n")
        .split("\n")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut fuel_reqs: Vec<u32> = Vec::new();

    for n in input {
        fuel_reqs.push((n / 3) - 2);
    }

    println!("{}", fuel_reqs.iter().fold(0, |a, b| a + b));
}
