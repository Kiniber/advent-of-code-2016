use std::{
    cmp::Ordering,
    sync::{Arc, RwLock},
};

use rayon::prelude::*;

pub fn part2(input: &str) -> anyhow::Result<()> {
    let pw_tracker = Arc::new(RwLock::new(vec![
        false, false, false, false, false, false, false, false,
    ]));
    let mut collected_hashs = std::iter::repeat(input)
        .enumerate()
        .par_bridge()
        .map(|(index, input)| (index, input, calculate_hash(index, input)))
        .filter(|(_, _, hash)| hash.starts_with("00000"))
        .filter_map(|(index, input, hash)| {
            let password_index = hash.chars().nth(5).to_owned();
            let password_char = hash.chars().nth(6).to_owned();
            if let (Some(password_index), Some(password_char)) = (password_index, password_char) {
                let parsed_pw_index = password_index.to_digit(10);
                return match parsed_pw_index {
                    Some(parsed_pw_index) => {
                        return Some((index, input, hash, parsed_pw_index, password_char));
                    }
                    None => None,
                };
            }
            None
        })
        .take_any_while(|(_index, _input, _hash, password_index, password_char)| {
            let c = match password_index {
                // Using @ Binding to limit only to indexes 0 - 7
                c @ 0..8 => Some(c),
                _ => None,
            };
            let Some(c) = c else {
                // this is the else part (Where c is None)
                let pw_tracker = pw_tracker.clone();
                // We don't have an index in range 0..8
                // We just check if we can continue or if some other thread already completed the search
                if let Ok(pw_tracker) = pw_tracker.read() {
                    // As long as the tracker contains a false, we continue
                    return pw_tracker.contains(&false);
                } else {
                    panic!("RwLock is poisoned!");
                }
            };
            // We actually found a index in range 0..8, lets update the tracker
            // We want write access to update the tracker
            if let Ok(mut pw_tracker) = pw_tracker.write() {
                // We update the corresponding index
                pw_tracker[*c as usize] = true;
                println!("found: {password_index}: '{password_char}'");
                // We always continue if we find something.
                // On the next iteration, we can check if we need to abort.
                // If we would exit here, the last find would not be included and a letter would be missing - ask me how I found out ;)
                return true;
            } else {
                panic!("RwLock is poisoned!")
            }
        })
        .collect::<Vec<_>>();

    println!("all found hashs (multiple for index):");
    collected_hashs.iter().for_each(print_result);
    println!("\nNow sorting for correct ordering...\n");
    collected_hashs.sort_by(|a, b| {
        // First compare the char_index of the letter
        let ord = a.3.cmp(&b.3);
        if ord == Ordering::Equal {
            // if the index is the same, the one found first has priority
            a.0.cmp(&b.0)
        } else {
            ord
        }
    });
    //let collected_hashs = collected_hashs.into_iter().collect::<Vec<_>>();

    //.map(|(_,_,_,_,relevant_char)|relevant_char)
    collected_hashs.iter().for_each(print_result);
    //println!("\n\nrelevant hashs: {collected_hashs:?}");
    let mut result = String::new();
    for index in 0u32..8u32 {
        let first_occurance = collected_hashs.iter().filter(|e| e.3 == index).next();
        if let Some(entry) = first_occurance {
            result.push(entry.4);
        }
    }
    println!("Result: {result}");
    Ok(())
}

fn calculate_hash(index: usize, input: &str) -> String {
    /*if index % 1_000_000 == 0 {
        println!("progress: current index = {index}");
    }*/
    let digest = md5::compute(format!("{input}{index}"));
    format!("{:x}", digest)
}

fn print_result(result: &(usize, &str, String, u32, char)) {
    println!("{}", format_result(result));
}

fn format_result(result: &(usize, &str, String, u32, char)) -> String {
    let (index, _input, hash, password_index, password_char) = result;
    format!("{index:10}: {hash} ({password_index}, '{password_char}')")
}
