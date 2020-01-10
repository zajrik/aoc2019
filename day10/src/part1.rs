#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x as f64,
            y: y as f64,
        }
    }

    fn equals(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Asteroid {
    location: Coord,
    others_in_los: usize,
}

impl Asteroid {
    fn new(location: Coord, others_in_los: usize) -> Asteroid {
        Asteroid {
            location,
            others_in_los,
        }
    }
}

pub fn part1() -> Result<Asteroid, std::io::Error> {
    let input: Vec<&str> = include_str!("./input.txt").split("\n").collect();
    let mut asteroid_locations: Vec<Coord> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                asteroid_locations.push(Coord::new(x, y));
            }
        }
    }

    let mut asteroids: Vec<Asteroid> = Vec::new();

    for a in &asteroid_locations {
        let mut asteroid_angles: Vec<f64> = Vec::new();

        for b in &asteroid_locations {
            if !a.equals(b) {
                asteroid_angles.push((b.y - a.y).atan2(b.x - a.x));
            }
        }

        asteroid_angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        asteroid_angles.dedup();
        asteroids.push(Asteroid::new(*a, asteroid_angles.len()))
    }

    asteroids.sort_by(|a, b| b.others_in_los.partial_cmp(&a.others_in_los).unwrap());
    Ok(asteroids[0])
}
