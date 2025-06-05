use std::fmt::Display;

use thiserror::Error;

pub fn part1(input: &str) -> anyhow::Result<()> {
    let input = input.trim();
    let chars = input.chars().collect::<Vec<_>>();
    let directions = chars
        .into_iter()
        .filter_map(|c| c.try_into().ok())
        .collect::<Vec<Direction>>();
    let mut current_key = Keypad::K5;

    let mut keys = Vec::new();
    let mut last_pushed_key= None;
    for dir in directions {
        if let Some(next_key) = &current_key.try_move(&dir) {
            current_key = next_key.clone();
            last_pushed_key = None;
            print!(" {dir}{current_key}");
        } else if last_pushed_key != Some(current_key) {
            keys.push(current_key);
            last_pushed_key = Some(current_key);
            println!("--> {dir}{current_key}");
        } else{
            println!("- {dir}{current_key} (ignored)");
        }
        // else the key was already pushed, ignore
    }
    println!("");
    if last_pushed_key.is_none() {
        keys.push(current_key);
        println!("--> {current_key} (final key)");
    }
    for key in keys {
        print!("{key}");
    }
    println!("");
    Ok(())
}

#[derive(Error, Debug)]
enum ApplicationError {
    #[error("The direction '{direction}' is not implemented")]
    UnsupportedDirectionError { direction: char },
}

#[derive(Error, Debug, Clone, Copy, PartialEq)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::U => f.write_str("U"),
            Direction::D => f.write_str("D"),
            Direction::L => f.write_str("L"),
            Direction::R => f.write_str("R"),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ApplicationError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::U),
            'D' => Ok(Direction::D),
            'L' => Ok(Direction::L),
            'R' => Ok(Direction::R),
            direction => Err(ApplicationError::UnsupportedDirectionError { direction }),
        }
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq)]
enum Keypad {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}

impl Display for Keypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keypad::K1 => f.write_str("1"),
            Keypad::K2 => f.write_str("2"),
            Keypad::K3 => f.write_str("3"),
            Keypad::K4 => f.write_str("4"),
            Keypad::K5 => f.write_str("5"),
            Keypad::K6 => f.write_str("6"),
            Keypad::K7 => f.write_str("7"),
            Keypad::K8 => f.write_str("8"),
            Keypad::K9 => f.write_str("9"),
                    }
    }
}

impl Keypad {
    fn try_move(&self, dir: &Direction) -> Option<Keypad> {
        match (self, dir) {
            (&Keypad::K1, &Direction::R) => Some(Keypad::K2),
            (&Keypad::K1, &Direction::D) => Some(Keypad::K4),

            (&Keypad::K2, &Direction::L) => Some(Keypad::K1),
            (&Keypad::K2, &Direction::R) => Some(Keypad::K3),
            (&Keypad::K2, &Direction::D) => Some(Keypad::K5),

            (&Keypad::K3, &Direction::L) => Some(Keypad::K2),
            (&Keypad::K3, &Direction::D) => Some(Keypad::K6),

            (&Keypad::K4, &Direction::U) => Some(Keypad::K1),
            (&Keypad::K4, &Direction::R) => Some(Keypad::K5),
            (&Keypad::K4, &Direction::D) => Some(Keypad::K7),

            (&Keypad::K5, &Direction::U) => Some(Keypad::K2),
            (&Keypad::K5, &Direction::L) => Some(Keypad::K4),
            (&Keypad::K5, &Direction::R) => Some(Keypad::K6),
            (&Keypad::K5, &Direction::D) => Some(Keypad::K8),

            (&Keypad::K6, &Direction::U) => Some(Keypad::K3),
            (&Keypad::K6, &Direction::L) => Some(Keypad::K5),
            (&Keypad::K6, &Direction::D) => Some(Keypad::K9),

            (&Keypad::K7, &Direction::U) => Some(Keypad::K4),
            (&Keypad::K7, &Direction::R) => Some(Keypad::K8),

            (&Keypad::K8, &Direction::U) => Some(Keypad::K5),
            (&Keypad::K8, &Direction::L) => Some(Keypad::K7),
            (&Keypad::K8, &Direction::R) => Some(Keypad::K9),

            (&Keypad::K9, &Direction::U) => Some(Keypad::K6),
            (&Keypad::K9, &Direction::L) => Some(Keypad::K8),

            (_, _) => None,
        }
    }
}
