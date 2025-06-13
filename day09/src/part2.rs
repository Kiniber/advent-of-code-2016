use std::{cmp, fmt::Display};
use inline_colorization::*;

pub fn part2(input: &str) -> anyhow::Result<()> {
    // Uncomment to test: The decompression length must be 445
    // let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
    let parsed = input.lines().flat_map(|line|line.split_whitespace())
        .map(|line|parse(line, 0)).flatten().collect::<Vec<_>>();
    parsed.iter().for_each(|fragment|println!("{fragment}"));
    let decompression_length: u128 = parsed.iter().map(|fragment|fragment.decompression_length()).sum();
    println!("Decompression length: {decompression_length}");
    Ok(())
}

enum Fragment {
    Compressed {
        level: usize,
        length: usize,
        repeat: usize,
        sub_fragments: Vec<Fragment>,
    },
    Uncompressed {
        level: usize,
        sequence: String
    }
}

impl Fragment {
    fn decompression_length(&self) -> u128 {
        match self {
            Fragment::Compressed { repeat, sub_fragments, .. } => {
                ((*repeat as u128)) * ( sub_fragments.iter()
                    .fold(0u128, |mut sum, sub_frag| {sum += sub_frag.decompression_length(); sum}))
            },
            Fragment::Uncompressed { sequence, .. } => sequence.len() as u128,
        }
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fragment::Compressed { level, length, repeat, sub_fragments } => {
                let indentation = " ".repeat(*level);
                let s = format!("{indentation}{color_red}({length}x{repeat})\n");
                f.write_str(s.as_str())?;
                for frag in sub_fragments {
                    frag.fmt(f)?;
                }
            },
            Fragment::Uncompressed { level, sequence } => {
                let indentation = " ".repeat(*level);
                let s = format!("{indentation}{color_blue}{sequence}{color_reset}\n");
                f.write_str(s.as_str())?;
            },
        };
        
        Ok(())
    }
}

fn parse(word: &str, level: usize) -> Vec<Fragment> {
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
            let fragment = Fragment::Uncompressed { level, sequence: uncompressed.to_string() } ;
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

        let sub_fragments = parse(repeated_part, level + 1);

        let fragment = Fragment::Compressed { level, length: a, repeat: b, sub_fragments };
        vec.push(fragment);
        
        //println!("\n\nrepeated_part: {repeated_part}\n\nremainder: {remainder}\n");
        
        // println!("{word}");
        // println!("{color_blue}{uncompressed}{color_red}{compressed}{color_reset}\n");
    }
    if remainder.len() > 0 {
        let fragment = Fragment::Uncompressed { level, sequence: remainder.to_string() } ;
        vec.push(fragment);
    }
    // println!("{color_green}{remainder}{color_reset}");

    vec
}