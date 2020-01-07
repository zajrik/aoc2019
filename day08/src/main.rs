mod part1;
mod part2;

use part1::*;
use part2::*;

fn main() -> Result<(), std::io::Error> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);
    Ok(())
}
