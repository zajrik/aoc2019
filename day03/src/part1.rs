use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0 };

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

/// Iterate over the given vector of WireSegments and return a HashSet containing
/// all points traversed by each segment, excluding the origin
fn process_wire_segments(segments: Vec<WireSegment>) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut start: Point = Point::ORIGIN;

    for segment in segments.iter() {
        match segment.direction {
            Direction::Up => {
                for n in start.x..=(start.x + segment.length as i32) {
                    points.insert(Point::new(n, start.y));
                }
                start = Point::new(start.x + segment.length as i32, start.y);
            }
            Direction::Down => {
                for n in ((start.x - segment.length as i32)..=start.x).rev() {
                    points.insert(Point::new(n, start.y));
                }
                start = Point::new(start.x - segment.length as i32, start.y);
            }
            Direction::Right => {
                for n in start.y..=(start.y + segment.length as i32) {
                    points.insert(Point::new(start.x, n));
                }
                start = Point::new(start.x, start.y + segment.length as i32);
            }
            Direction::Left => {
                for n in ((start.y - segment.length as i32)..=start.y).rev() {
                    points.insert(Point::new(start.x, n));
                }
                start = Point::new(start.x, start.y - segment.length as i32);
            }
        }
    }

    points.remove(&Point::ORIGIN);
    points
}

pub fn part1() -> Option<()> {
    let mut input: Vec<Vec<WireSegment>> = include_str!("./input.txt")
        .replace("\r\n", "\n")
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|segment| WireSegment::from(segment))
                .collect()
        })
        .collect();

    let wire_1: HashSet<Point> = process_wire_segments(input.pop()?);
    let wire_2: HashSet<Point> = process_wire_segments(input.pop()?);

    let closest_to_origin: i32 = wire_1
        .intersection(&wire_2)
        .map(|p| p.x.abs() + p.y.abs())
        .min()?;

    println!("{}", closest_to_origin);
    Some(())
}
