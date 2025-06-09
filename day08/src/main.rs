use std::time::Instant;

mod part1and2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 8.1 + 8.2");
    let input = include_str!("../resources/input.txt").trim();
    let start_time = Instant::now();
    part1and2::part1and2(input)?;
    let duration = start_time.elapsed();
    println!("8.1 + 8.2 Time elapsed: {duration:?}");
    Ok(())
}
