use crate::file;
use std::{collections::HashSet, io::BufRead};

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day5.txt")?;
    let mut fresh: u64 = 0;
    let mut not_spoiled = HashSet::<String>::new();
    let mut at_ids = false;
    for line in reader.lines().map(|l| l.unwrap()) {
        if line == "" {
            at_ids = true;
            continue;
        }
        if at_ids {
            for k in &not_spoiled {
                let parts = k.split_once('-').ok_or("no dash?")?;
                let (from, to): (u64, u64) = (parts.0.parse()?, parts.1.parse()?);
                let l: u64 = line.parse()?;
                if l >= from && l <= to {
                    fresh += 1;
                    break;
                }
            }
        } else {
            not_spoiled.insert(line);
        }
    }

    assert_eq!(643, fresh);
    println!("Day 5 part 1 passed");
    Ok(())
}
