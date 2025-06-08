use inline_colorization::*;

pub fn part1(input: &str) -> anyhow::Result<()> {
    let valid_amount = input.split("\n").filter(|line|{
        let mut chars = line.chars();
        let mut sequence_valid = true;
        let mut bracket = None;
        let mut a_opt = chars.next();
        let mut b_opt = chars.next();
        let mut c_opt = chars.next();
        let mut d_opt = chars.next();
        let mut everything_valid = false;
        let mut found_sequence = String::new();
        while let (opening_bracket, Some(a), Some(b), Some(c), Some(d)) = (bracket ,a_opt, b_opt, c_opt, d_opt) {
            if opening_bracket == Some('[') {
                sequence_valid = false;
            } else if opening_bracket == Some(']') {
                sequence_valid = true;
            }
            if sequence_valid && a==d && b==c && a!=b {
                found_sequence = format!("{a}{b}{c}{d}");
                println!("{found_sequence} - {}",line.replace(found_sequence.as_str(), format!("{color_green}{found_sequence}{color_reset}").as_str()));
                everything_valid = true;
            }
            if !sequence_valid && a==d && b==c && a!=b {
                let bad_sequence = format!("{a}{b}{c}{d}");
                if everything_valid {
                    println!("{color_red}!!!!!!!!!! This is the bad guy !!!!!!!!!!!!!!!{color_reset}");
                    println!("{bad_sequence} - {}",
                    line.replace(bad_sequence.as_str(), format!("{color_red}{bad_sequence}{color_reset}").as_str())
                        .replace(found_sequence.as_str(), format!("{color_green}{bad_sequence}{color_reset}").as_str()));
                    println!("{color_red}!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{color_reset}");
                } else {
                    println!("{bad_sequence} - {}",
                        line.replace(bad_sequence.as_str(), format!("{color_red}{bad_sequence}{color_reset}").as_str()));
                }
                everything_valid = false;
                return false;
            }
            bracket = a_opt;
            a_opt = b_opt;
            b_opt = c_opt;
            c_opt = d_opt;
            d_opt = chars.next();
        }
        everything_valid
    }).count();
    println!("Amount of IPs supporting TLS (transport-layer snooping): {valid_amount}");
    Ok(())
}
