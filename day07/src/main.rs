use std::time::Instant;

mod part1;
mod part2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 7.1");
    let input = include_str!("../resources/input.txt").trim();
    let start_time = Instant::now();
    part1::part1(input)?;
    let duration = start_time.elapsed();
    println!("7.1 Time elapsed: {duration:?}");
    
    println!("Advent of code 2016: 7.2");
    let start_time = Instant::now();
    part2::part2(input)?;
    let duration = start_time.elapsed();
    println!("7.2 Time elapsed: {duration:?}");
    Ok(())
}
