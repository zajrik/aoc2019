#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct WireSegment {
    direction: Direction,
    length: u32,
}

impl WireSegment {
    fn from(input: &str) -> WireSegment {
        let input_chars: Vec<char> = input.chars().collect();

        let direction: Direction = match input_chars[0] {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid input"),
        };

        let length: u32 = input_chars[1..]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        WireSegment { direction, length }
    }
}

// struct Coord {
//     x: i32,
//     y: i32,
// }

pub fn part1() {
    let mut input: Vec<Vec<WireSegment>> = include_str!("./input.txt")
        .replace("\r\n", "\n")
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|segment| WireSegment::from(segment))
                .collect()
        })
        .collect();

    let wire2: Vec<WireSegment> = input.pop().unwrap();
	let wire1: Vec<WireSegment> = input.pop().unwrap();
	
	let test: bool = wire1.contains(&WireSegment { direction: Direction::Right, length: 352 });

	println!("{}", test);

    println!("{:?}", wire1[0]);
    println!("{:?}", wire2[0]);
}
