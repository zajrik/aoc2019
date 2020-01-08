pub fn part1() -> Result<usize, std::io::Error> {
    let input: &str = include_str!("./input.txt");

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const LAYER_SIZE: usize = HEIGHT * WIDTH;

    let layers: Vec<Vec<char>> = vec![0; input.len() / LAYER_SIZE]
        .iter()
        .enumerate()
        .map(|(i, _)| {
            input[(LAYER_SIZE * i)..(LAYER_SIZE * (i + 1))]
                .chars()
                .collect()
        })
        .collect();

    let mut least_zeroes: (usize, u32) = (0, std::u32::MAX);

    for (i, layer) in layers.iter().enumerate() {
        let zero_count: u32 = layer.iter().filter(|n| **n == '0').count() as u32;

        if zero_count < least_zeroes.1 {
            least_zeroes = (i, zero_count);
        }
    }

    let ones: usize = layers[least_zeroes.0].iter().filter(|n| **n == '1').count();
    let twos: usize = layers[least_zeroes.0].iter().filter(|n| **n == '2').count();

    Ok(ones * twos)
}
