pub fn part2() -> Result<String, std::io::Error> {
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

    let mut result: Vec<Option<char>> = vec![None; LAYER_SIZE];

    for layer in layers {
        for (i, pixel) in layer.iter().enumerate() {
            if result[i].is_none() {
                result[i] = match *pixel {
                    '1' => Some('#'),
                    '0' => Some(' '),
                    _ => None,
                }
            }
        }
    }

    let output: String = vec![0; result.len() / WIDTH]
        .iter()
        .enumerate()
        .map(|(i, _)| {
            result[(WIDTH * i)..(WIDTH * (i + 1))]
                .iter()
                .map(|n| n.unwrap_or(' '))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(format!("\n{}", output))
}
