use std::time::Instant;

mod part1;
mod part1_single_threaded;
mod part2;
mod part2_single;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 5.1");
    let input = include_str!("../resources/input.txt").trim();
    let start_time = Instant::now();
    part1::part1(input)?;
    let duration = start_time.elapsed();
    println!("5.1 Time elapsed (parallel): {duration:?}");

    let start_time = Instant::now();
    part1_single_threaded::part1(input)?;
    let duration = start_time.elapsed();
    println!("5.1 Time elapsed (single threaded): {duration:?}");
    println!("Advent of code 2016: 5.2");
    let start_time = Instant::now();
    part2::part2(input)?;
    let duration = start_time.elapsed();
    println!("5.2 Time elapsed (parallel): {duration:?}");
    println!("Advent of code 2016: 5.2");
    let start_time = Instant::now();
    part2_single::part2(input)?;
    let duration = start_time.elapsed();
    println!("5.2 Time elapsed (single threaded): {duration:?}");
    Ok(())
}
