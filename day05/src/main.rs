use std::time::Instant;

mod part1;
mod part1_single_threaded;
mod part2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 5.1");
    let input = include_str!("../resources/input.txt").trim();
    let start_time = Instant::now();
    part1::part1(input)?;
    let duration = start_time.elapsed();
    println!("Time elapsed (parallel): {duration:?}");

    let start_time = Instant::now();
    part1_single_threaded::part1(input)?;
    let duration = start_time.elapsed();
    println!("Time elapsed (single threaded): {duration:?}");
    //println!("Advent of code 2016: 5.2");
    //part2::part2(input)?;
    Ok(())
}
