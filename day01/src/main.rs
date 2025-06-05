mod part1;

use anyhow::Ok;
use part1::*;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 1.1");

    let input_1_1 = include_str!("../resources/input_1_1.txt");
    
    part1(input_1_1)?;

    Ok(())
}
