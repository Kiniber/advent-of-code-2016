mod part1;
mod part2;

use anyhow::Ok;
use part1::part1;
use part2::part2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 1.1");
    let input_1_1 = include_str!("../resources/input_1_1.txt");
    part1(input_1_1)?;
    println!("Advent of code 2016: 1.2");
    part2(input_1_1)?;
    Ok(())
}
