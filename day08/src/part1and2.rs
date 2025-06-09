use std::{fmt::Display, str::FromStr};
use thiserror::Error;
use inline_colorization::*;

pub fn part1and2(input: &str) -> anyhow::Result<()> {
    let operations = input.split("\n").map(DisplayOperation::from_str).filter_map(|e|e.ok()).collect::<Vec<_>>();
    let mut display = Display1::new(50, 6);
    for operation in operations {
        display.apply_operation(operation.clone());
        println!("Operation: {operation:?}): {display}");
    }
    println!("Display (#Pixels on: {}): {display}", display.get_lit_pixels());
    /*
    println!("Display: {display}");
    display.turn_on_rect(3, 2);
    println!("Display: {display}");
    display.rotate_column_down(1, 2);
    println!("Display: {display}");
    display.rotate_row_right(1, 4);
    println!("Display: {display}");
     */

    Ok(())
}

#[derive(Debug,Clone)]
enum DisplayOperation {
    Rect{x: usize, y: usize},
    RotateRow{y: usize, amount: usize},
    RotateColumn{x: usize, amount: usize}
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Unable to parse display operation from: {0}")]
    DisplayParseError(String),
}

impl FromStr for DisplayOperation {
    type Err = ApplicationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split_whitespace().collect::<Vec<_>>();
        if words.len() == 2 {
            if words[0] == "rect" {
                let dims = words[1].split("x").collect::<Vec<_>>();
                if let (Ok(x), Ok(y)) = (dims[0].parse::<usize>(), dims[1].parse::<usize>()){
                    return Ok(DisplayOperation::Rect { x, y });
                }
            }
        }
        if words.len() == 5 {
            if words[0] == "rotate" {
                if words[1] == "row" {
                    if let Some(y) = words[2].split("=").nth(1) {
                        if let (Ok(y), Ok(amount)) = (y.parse::<usize>(), words[4].parse::<usize>()) {
                            return Ok(DisplayOperation::RotateRow { y, amount });
                        }
                    }
                } else if words[1] == "column" {
                    if let Some(x) = words[2].split("=").nth(1) {
                        if let (Ok(x), Ok(amount)) = (x.parse::<usize>(), words[4].parse::<usize>()) {
                            return Ok(DisplayOperation::RotateColumn { x, amount });
                        }
                    }
                }
            }
        }
        Err(ApplicationError::DisplayParseError(line.to_string()))
    }
}

struct Display1 {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>
}

impl Display1 {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![false; height]; width],
        }
    }

    fn apply_operation(&mut self, operation: DisplayOperation) {
        match operation {
            DisplayOperation::Rect { x, y } => self.turn_on_rect(x, y),
            DisplayOperation::RotateRow { y, amount } => self.rotate_row_right(y, amount),
            DisplayOperation::RotateColumn { x, amount } => self.rotate_column_down(x, amount),
        }
    }

    fn turn_on_rect(&mut self, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                self.pixels[x][y] = true;
            }
        }
    }

    fn rotate_column_down(&mut self, column_index: usize, rotation_amount: usize) {
        let mut new_column = vec![false; self.height];
        for y in 0..self.height {
            new_column[y] = self.pixels[column_index][(self.height + y - rotation_amount) % self.height];
        }
        for y in 0..self.height {
            self.pixels[column_index][y] = new_column[y];
        }
    }

    fn rotate_row_right(&mut self, row_index: usize, rotation_amount: usize) {
        let mut new_row = vec![false; self.width];
        for x in 0..self.width {
            new_row[x] = self.pixels[(self.width + x - rotation_amount) % self.width][row_index];
        }
        for x in 0..self.width {
            self.pixels[x][row_index] = new_row[x];
        }
    }

    fn get_lit_pixels(&self) -> usize {
        let mut amount = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.pixels[x][y] {
                    amount += 1;
                }
            }
        }
        amount
    }
}

impl Display for Display1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = "\n".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.pixels[x][y] {
                    builder.push_str(format!("{color_red}X{color_reset}").as_str());
                    
                } else {
                    builder.push('O');
                }
            }
            builder.push('\n');
        }
        builder.push('\n');
        write!(f, "{builder}")
    }
}