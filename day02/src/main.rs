mod part1;
mod part2;

use part1::part1;
use part2::part2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 2.1");
    let input = include_str!("../resources/input_2_1.txt");
    part1(input)?;
    println!("Advent of code 2016: 2.2");
    part2(input)?;
    Ok(())
}
