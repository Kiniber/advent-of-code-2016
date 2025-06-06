use itertools::Itertools;

pub fn part1(input: &str) -> anyhow::Result<()> {
    let all_shapes = input.lines()
        .map(|line| {
            let parts = line.split_whitespace()
            // collect_tuple is from the crate itertools
            .collect_tuple::<(&str, &str, &str)>()
            // We can iterate over an option
            // it is like a list with either 0 or 1 entries
            .iter()
            // convert the strings to numbers and keep the tuple form
            .map_while(|(a,b,c)|{
                match (a.parse::<i32>(), b.parse::<i32>(), c.parse::<i32>()) {
                    // if all 3 conversions were ok, we have a result
                    (Ok(a), Ok(b), Ok(c)) => 
                    {
                        let mut v = [a, b, c];
                        v.sort(); // We sort the values
                        let [a,b,c] = v;
                        Some((a,b,c))
                    },
                    // Since we use map_while, None values will be removed from the stream
                    _ => None
                }
            }).collect::<Vec<_>>();
            parts
        })
        .flatten()
        .collect::<Vec<_>>();
    //println!("{all_shapes:?}");
    let amount_triangles = all_shapes.iter().filter(ValidateTriangle::is_triangle).count();
    println!("Found {amount_triangles} triangles");
    Ok(())
}

trait ValidateTriangle {
    fn is_triangle(&self) -> bool;
}

impl ValidateTriangle for &(i32, i32, i32) {
    fn is_triangle(&self) -> bool {
        // We sorted earlier, so c is the biggest
        let (a,b,c) = self;
        c - a - b < 0
    }
}
