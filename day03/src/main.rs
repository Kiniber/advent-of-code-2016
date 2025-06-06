mod part1;

use part1::part1;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 3.1");
    let input = include_str!("../resources/input.txt");
    part1(input)?;
    Ok(())
}
