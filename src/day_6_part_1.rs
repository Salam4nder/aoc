use crate::file;
use std::{collections::HashMap, io::BufRead};

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day6.txt")?;
    let mut map: HashMap<usize, Vec<u128>> = HashMap::new();
    let mut op_map: HashMap<usize, Op> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();
    let len = lines.len();
    for (i, line) in lines.iter().enumerate() {
        if i == len - 1 {
            for (i, v) in line.split_whitespace().enumerate() {
                let op = Op::from_str(v)?;
                op_map.entry(i).or_insert(op);
            }
        } else {
            for (i, v) in line.split_whitespace().enumerate() {
                let v: u128 = v.parse()?;
                map.entry(i)
                    .and_modify(|vec| vec.push(v))
                    .or_insert(vec![v]);
            }
        }
    }

    let mut total_vec: Vec<u128> = vec![];
    for i in 0..op_map.len() {
        let op = op_map.get(&i).ok_or::<String>("wtf".into())?;
        let vec = map.get(&i).ok_or::<String>("wtff".into())?;
        match op {
            Op::Plus => {
                let mut total: u128 = 0;
                for v in vec {
                    total += *v;
                }
                total_vec.push(total);
            }
            Op::Mul => {
                let mut total: u128 = 1;
                for v in vec {
                    total *= *v;
                }
                total_vec.push(total);
            }
        }
    }
    let mut total: u128 = 0;
    for v in total_vec {
        total += v;
    }
    assert_eq!(6172481852142, total);
    println!("Day 6 part 1 passed");
    Ok(())
}

#[derive(Debug)]
enum Op {
    Plus,
    Mul,
}

impl Op {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Mul),
            _ => Err("not an op".to_string()),
        }
    }
}
