use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day3.txt")?;
    let mut total: u64 = 0;
    let mut outer_vec = vec![];

    for line in reader.lines() {
        let mut inner_vec = vec![];
        let line = line?;
        let mut chars: Vec<char> = line.chars().collect();
        let mut chars_left = 12;
        while inner_vec.len() != 12 {
            let mut largest: u32 = 0;
            let mut broken_index = 0;
            let len = chars.len();
            for (i, c) in chars.iter().enumerate() {
                if len - i + 1 <= chars_left {
                    break;
                }
                let c = c.to_digit(10).expect("it's a digit");
                if c > largest {
                    largest = c;
                    broken_index = i;
                }
            }
            for _ in 0..=broken_index {
                chars.remove(0);
            }
            inner_vec.push(largest);
            chars_left -= 1;
        }
        outer_vec.push(inner_vec);
    }
    for v in outer_vec {
        let accum = v.iter().fold(0u64, |acc, d| acc * 10 + u64::from(*d));
        total += accum;
    }

    assert_eq!(172516781546707, total);
    println!("Day 3 part 2 passed");
    Ok(())
}
