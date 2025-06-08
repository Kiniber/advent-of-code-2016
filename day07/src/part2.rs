use std::collections::HashSet;

use inline_colorization::*;

pub fn part2(input: &str) -> anyhow::Result<()> {
    let valid_amount = input
        .split("\n")
        .filter(|line| {
            let mut supernet = HashSet::new();
            let mut hypernet = HashSet::new();
            let mut chars = line.chars();
            let mut is_supernet = true;
            let mut bracket = None;
            let mut a_opt = chars.next();
            let mut b_opt = chars.next();
            let mut c_opt = chars.next();
            while let (opening_bracket, Some(a), Some(b), Some(c)) = (bracket, a_opt, b_opt, c_opt)
            {
                if opening_bracket == Some('[') {
                    is_supernet = false;
                } else if opening_bracket == Some(']') {
                    is_supernet = true;
                }
                if a == c && a!=b {
                    let aba_sequence = format!("{a}{b}{c}");
                    if is_supernet {
                        supernet.insert(aba_sequence);
                    } else {
                        hypernet.insert(aba_sequence);
                    }
                }
                bracket = a_opt;
                a_opt = b_opt;
                b_opt = c_opt;
                c_opt = chars.next();
            }
            for aba in supernet {
                let mut aba_chars = aba.chars();
                let a = aba_chars.next().unwrap();
                let b = aba_chars.next().unwrap();
                let bab = format!("{b}{a}{b}");
                //println!("checking: {aba},{bab}");
                if hypernet.contains(&bab) {
                    println!(
                        "{aba},{bab} - {}",
                        line.replace(&aba, format!("{color_blue}{aba}{color_reset}").as_str())
                            .replace(&bab, format!("{color_red}{bab}{color_reset}").as_str())
                    );
                    return true;
                }
            }
            false
        })
        .count();
    println!("Amount of IPs supporting SSL (super-secret listening): {valid_amount}");
    Ok(())
}
