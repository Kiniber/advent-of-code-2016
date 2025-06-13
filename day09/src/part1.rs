use std::{cmp, fmt::Display};
use inline_colorization::*;

pub fn part1(input: &str) -> anyhow::Result<()> {
    let parsed = input.lines().flat_map(|line|line.split_whitespace()).map(parse).flatten().collect::<Vec<_>>();
    parsed.iter().for_each(|fragment|println!("{fragment}"));
    let decompression_length: usize = parsed.iter().map(|fragment|fragment.decompression_length()).sum();
    println!("Decompression length: {decompression_length}");
    Ok(())
}

enum Fragment {
    Compressed {
        length: usize,
        repeat: usize,
        sequence: String,
    },
    Uncompressed {
        sequence: String
    }
}

impl Fragment {
    fn decompression_length(&self) -> usize {
        match self {
            Fragment::Compressed { length, repeat, sequence } => cmp::max(sequence.len(), length * repeat),
            Fragment::Uncompressed { sequence } => sequence.len(),
        }
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Fragment::Compressed { length, repeat, sequence } => format!("{color_red}({repeat}x{length}){color_magenta}{sequence}{color_reset}"),
            Fragment::Uncompressed { sequence } => format!("{color_blue}{sequence}{color_reset}"),
        };
        f.write_str(str.as_str())
    }
}

fn parse(word: &str) -> Vec<Fragment> {
    // ^( \(\d+x\d+\) )
    let regex = regex::Regex::new(r"(?x)
    (?<uncompressed>.*?) # not (axb) any amount of times 
    (\((?<compressed>\d+x\d+)\)){1} # (axb) exactly once
    (?<remainder>.*) # whatever is left afterwards"
    ).expect("Compiling Regex failed");
    let mut vec = vec![];
    let mut uncompressed;
    let mut compressed;
    let mut remainder = word;
    while let Some(captures) =regex.captures(remainder) {

        uncompressed = captures.name("uncompressed").map(|m|m.as_str()).unwrap_or_default();
        if uncompressed.len() > 0 {
            let fragment = Fragment::Uncompressed { sequence: uncompressed.to_string() } ;
            vec.push(fragment);
        }
        compressed = captures.name("compressed").map(|m|m.as_str()).unwrap();
        let axb = compressed.split("x").collect::<Vec<_>>();
        let a = axb[0].parse::<usize>().expect("Regex does not work");
        let b = axb[1].parse::<usize>().expect("Regex does not work");

        remainder = captures.name("remainder").map(|m|m.as_str()).unwrap_or_default();

        let char_amount = cmp::min(a, remainder.len());
        let repeated_part = &remainder[0..char_amount];
        remainder = &remainder[char_amount..];

        let fragment = Fragment::Compressed { length: a, repeat: b, sequence: repeated_part.to_string() };
        vec.push(fragment);
        
        //println!("\n\nrepeated_part: {repeated_part}\n\nremainder: {remainder}\n");
        
        // println!("{word}");
        // println!("{color_blue}{uncompressed}{color_red}{compressed}{color_reset}\n");
    }
    if remainder.len() > 0 {
        let fragment = Fragment::Uncompressed { sequence: remainder.to_string() } ;
        vec.push(fragment);
    }
    // println!("{color_green}{remainder}{color_reset}");

    vec
}