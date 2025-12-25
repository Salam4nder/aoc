use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day5.txt")?;
    let mut seen: Vec<(u128, u128)> = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        if line == "" {
            break;
        }
        let parts = line.split_once('-').ok_or("no dash?")?;
        let (from, to): (u128, u128) = (parts.0.parse()?, parts.1.parse()?);
        seen.push((from, to));
    }
    seen.sort();
    let total = find_in_tree(seen);

    assert_eq!(342018167474526, total);
    println!("Day 5 part 2 passed");
    Ok(())
}

fn find_in_tree(vec: Vec<(u128, u128)>) -> u128 {
    let mut total = 0;
    let mut last: u128 = 0;
    for (k, v) in vec {
        if last > v {
            continue;
        }
        total += v - last.max(k) + 1;
        last = v + 1;
    }
    total
}
