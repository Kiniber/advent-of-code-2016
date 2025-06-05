mod part1;

use part1::part1;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 2.1");
    let input_1_1 = include_str!("../resources/input_2_1.txt");
    part1(input_1_1)?;
    Ok(())
}
