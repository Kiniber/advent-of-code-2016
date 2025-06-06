use thiserror::Error;

pub fn part2(input: &str) -> anyhow::Result<()> {
    let input = input.trim();
    let lines = input.split("\n").collect::<Vec<_>>();
    let mut keys = Vec::new();
    for line in lines {
        analyze_line(line, &mut keys)?;
    }
    for key in keys {
        print!("{key}");
    }
    println!("");
    Ok(())
}

fn analyze_line(input: &str, keys: &mut Vec<Keypad>) -> anyhow::Result<()> {
    let chars = input.chars().collect::<Vec<_>>();
    let directions = chars
        .into_iter()
        .filter_map(|c| c.to_string().as_str().try_into().ok())
        .collect::<Vec<Direction>>();
    let mut current_key = Keypad::K5;
    let mut last_pushed_key= None;
    for dir in directions {
        if let Some(next_key) = &current_key.try_move(&dir) {
            current_key = next_key.clone();
            last_pushed_key = None;
            //print!(" {dir}{current_key}");
        } else if last_pushed_key != Some(current_key) {
            //keys.push(current_key);
            last_pushed_key = Some(current_key);
            //println!(" --> {dir}{current_key} (hitting edge)");
        } else{
            //println!(" --> {dir}{current_key} (hitting edge again)");
        }
        // else the key was already pushed, ignore
    }
    println!("");
    keys.push(current_key);
    println!("--> {current_key} (final key)");
    
    Ok(())
}

#[derive(Error, Debug, Clone, Copy, PartialEq, strum_macros::Display , strum_macros::EnumString)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, strum_macros::Display)]
enum Keypad {
    #[strum(to_string = "1")]
    K1,
    #[strum(to_string = "2")]
    K2,
    #[strum(to_string = "3")]
    K3,
    #[strum(to_string = "4")]
    K4,
    #[strum(to_string = "5")]
    K5,
    #[strum(to_string = "6")]
    K6,
    #[strum(to_string = "7")]
    K7,
    #[strum(to_string = "8")]
    K8,
    #[strum(to_string = "9")]
    K9,
    #[strum(to_string = "A")]
    KA,
    #[strum(to_string = "B")]
    KB,
    #[strum(to_string = "C")]
    KC,
    #[strum(to_string = "D")]
    KD,
}

impl Keypad {
    fn try_move(&self, dir: &Direction) -> Option<Keypad> {
        match (self, dir) {
            (&Keypad::K1, &Direction::D) => Some(Keypad::K3),

            (&Keypad::K2, &Direction::R) => Some(Keypad::K3),
            (&Keypad::K2, &Direction::D) => Some(Keypad::K6),

            (&Keypad::K3, &Direction::U) => Some(Keypad::K1),
            (&Keypad::K3, &Direction::D) => Some(Keypad::K7),
            (&Keypad::K3, &Direction::L) => Some(Keypad::K2),
            (&Keypad::K3, &Direction::R) => Some(Keypad::K4),

            (&Keypad::K4, &Direction::L) => Some(Keypad::K3),
            (&Keypad::K4, &Direction::D) => Some(Keypad::K8),

            (&Keypad::K5, &Direction::R) => Some(Keypad::K6),

            (&Keypad::K6, &Direction::U) => Some(Keypad::K2),
            (&Keypad::K6, &Direction::D) => Some(Keypad::KA),
            (&Keypad::K6, &Direction::L) => Some(Keypad::K5),
            (&Keypad::K6, &Direction::R) => Some(Keypad::K7),

            (&Keypad::K7, &Direction::U) => Some(Keypad::K3),
            (&Keypad::K7, &Direction::D) => Some(Keypad::KB),
            (&Keypad::K7, &Direction::L) => Some(Keypad::K6),
            (&Keypad::K7, &Direction::R) => Some(Keypad::K8),

            (&Keypad::K8, &Direction::U) => Some(Keypad::K4),
            (&Keypad::K8, &Direction::D) => Some(Keypad::KC),
            (&Keypad::K8, &Direction::L) => Some(Keypad::K7),
            (&Keypad::K8, &Direction::R) => Some(Keypad::K9),

            (&Keypad::K9, &Direction::L) => Some(Keypad::K8),

            (&Keypad::KA, &Direction::U) => Some(Keypad::K6),
            (&Keypad::KA, &Direction::R) => Some(Keypad::KB),

            (&Keypad::KB, &Direction::U) => Some(Keypad::K7),
            (&Keypad::KB, &Direction::D) => Some(Keypad::KD),
            (&Keypad::KB, &Direction::L) => Some(Keypad::KA),
            (&Keypad::KB, &Direction::R) => Some(Keypad::KC),

            (&Keypad::KC, &Direction::U) => Some(Keypad::K8),
            (&Keypad::KC, &Direction::L) => Some(Keypad::KB),

            (&Keypad::KD, &Direction::U) => Some(Keypad::KB),

            (_, _) => None,
        }
    }
}
