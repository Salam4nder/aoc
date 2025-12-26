use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day6.txt")?;
    let lines: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();
    let line_len = lines.first().unwrap().len();
    let column_vertical = lines.len();

    let mut total: u128 = 0;
    let mut sum: u128 = 0;
    let mut mul: u128 = 1;
    let mut is_mul: Option<bool> = None;
    for column_horizontal in 0..line_len {
        let mut s = String::new();
        for i in 0..column_vertical {
            let line = lines.get(i).unwrap();
            for c in line.chars().rev().skip(column_horizontal) {
                if !c.is_whitespace() {
                    if let Ok(o) = Op::from_char(c) {
                        match o {
                            Op::Mul => {
                                is_mul = Some(true);
                            }
                            Op::Plus => {
                                is_mul = Some(false);
                            }
                        }
                    } else {
                        s.push(c);
                    }
                }
                break;
            }
        }
        if !s.is_empty() {
            let v: u128 = s.parse()?;
            mul *= v;
            sum += v;
        }
        if let Some(b) = is_mul {
            if b {
                total += mul
            } else {
                total += sum
            }
            mul = 1;
            sum = 0;
            is_mul = None;
        }
    }
    assert_eq!(10188206723429, total);
    println!("Day 6 part 2 passesd");

    Ok(())
}

#[derive(Debug)]
enum Op {
    Plus,
    Mul,
}

impl Op {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '+' => Ok(Self::Plus),
            '*' => Ok(Self::Mul),
            _ => Err("not an op".to_string()),
        }
    }
}
