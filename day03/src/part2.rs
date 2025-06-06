use itertools::Itertools;

pub fn part2(input: &str) -> anyhow::Result<()> {

    let words = input.split_whitespace().collect::<Vec<_>>();
    let chunked = words.into_iter().chunks(9);
    //let chunked = chunked.into_iter().collect::<Vec<_>>();
    let all_shapes = chunked.into_iter().map(|mut chunk|{
        let a1 = chunk.next().unwrap();
        let a2 = chunk.next().unwrap();
        let a3 = chunk.next().unwrap();
        let a4 = chunk.next().unwrap();
        let a5 = chunk.next().unwrap();
        let a6 = chunk.next().unwrap();
        let a7 = chunk.next().unwrap();
        let a8 = chunk.next().unwrap();
        let a9 = chunk.next().unwrap();

        vec![
            (a1, a4, a7),
            (a2, a5, a8),
            (a3, a6, a9),
        ]
    }).flatten()
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
