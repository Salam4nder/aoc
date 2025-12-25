use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day2.txt")?;
    let mut password: u64 = 0;
    let pairs = reader.split(b',');
    pairs.for_each(|v| {
        let v = v.expect("to bytes");
        let s = String::from_utf8(v).expect("UTF8");
        let s = s.trim();
        let (from, to) = s.split_once('-').expect("split once");
        let from: u64 = from.parse().expect("it's a number");
        let to = to.parse().expect("it's a number");
        for v in from..=to {
            let len = if v == 0 { 1 } else { v.ilog10() + 1 };
            if len % 2 == 0 {
                let divisor = 10u64.pow(len / 2);
                let left = v / divisor;
                let right = v % divisor;
                if left == right {
                    password += v;
                }
            }
        }
    });
    assert_eq!(23701357374, password);
    println!("Day 2 part 1 passed");
    Ok(())
}
