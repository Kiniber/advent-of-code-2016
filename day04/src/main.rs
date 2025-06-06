mod part1;
mod part2;

use part1::part1;
use part2::part2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 4.1");
    let input = include_str!("../resources/input.txt").trim();
    part1(input)?;
    //println!("Advent of code 2016: 4.2");
    //part2(input)?;
    Ok(())
}
