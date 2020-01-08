pub fn part1() -> Result<usize, std::io::Error> {
    let input: &str = include_str!("./input.txt");
    let layer_size: usize = 25 * 6;
    let layers: Vec<Vec<u8>> = vec![""; input.len() / layer_size]
        .iter()
        .enumerate()
        .map(|(i, _)| {
            input[(layer_size * i)..(layer_size * (i + 1))]
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let mut least_zeroes: (usize, u32) = (0, std::u32::MAX);

    for layer in layers.iter().enumerate() {
        let zero_count: u32 = layer.1.iter().filter(|n| **n == 0).count() as u32;
        if zero_count < least_zeroes.1 {
            least_zeroes = (layer.0, zero_count);
        }
    }

    let ones: usize = layers[least_zeroes.0].iter().filter(|n| **n == 1).count();
    let twos: usize = layers[least_zeroes.0].iter().filter(|n| **n == 2).count();

    Ok(ones * twos)
}
