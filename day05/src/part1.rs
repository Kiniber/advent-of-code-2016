use rayon::prelude::*;

pub fn part1(input: &str) -> anyhow::Result<()> {
    //let input = "abc";
    let mut collected_hashs = 
        std::iter::repeat(input).enumerate()
        .par_bridge()
    .map(|(index, input)| (index, input, calculate_hash(index, input)))
    .filter(|(_, _, hash)| hash.starts_with("00000"))
    .map(|(index, input, hash)|{
        let slice = hash.chars().nth(5);
        let relevant_char = slice.to_owned();
        (index, input, hash, relevant_char.clone())
    })
    .take_any(8)
    //.take_any_while(|(index,input, hash)| index < &5)
    
    .collect::<Vec<_>>()
    ;
    collected_hashs.sort_by(|a, b| a.0.cmp(&b.0));
    println!("hashs: {collected_hashs:?}");
    let result = collected_hashs.into_iter().filter_map(|(_,_,_,relevant_char)|relevant_char)
    .fold(String::new(), |mut builder, val| {
        builder.push(val);
        builder
    });
    println!("Result: {result}");
    Ok(())
}

fn calculate_hash(index: usize, input: &str) -> String {
    if index % 1_000_000 == 0 {
        println!("progress: current index = {index}");
    }
    let digest = md5::compute(format!("{input}{index}"));
    format!("{:x}", digest)
} 