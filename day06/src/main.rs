use std::time::Instant;

mod part1;
// mod part2;

fn main() -> anyhow::Result<()> {
    println!("Advent of code 2016: 6.1");
    let input = include_str!("../resources/input.txt").trim();
    let start_time = Instant::now();
    part1::part1(input)?;
    let duration = start_time.elapsed();
    println!("6.1 Time elapsed: {duration:?}");
    /*
    println!("Advent of code 2016: 6.2");
    let start_time = Instant::now();
    part2::part2(input)?;
    let duration = start_time.elapsed();
    println!("6.2 Time elapsed (parallel): {duration:?}");
    */
    Ok(())
}
