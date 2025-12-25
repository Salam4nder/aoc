use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day1.txt")?;

    let mut safe = Safe::new();
    let mut pass: u16 = 0;
    for line in reader.lines() {
        let line = line?;
        let (direction, amount) = parse_op(&line);
        for _ in 0..amount {
            if direction == "L" {
                safe.rotate_left();
            } else {
                safe.rotate_right();
            }
        }
        if safe.dial == 0 {
            pass += 1;
        }
    }
    assert_eq!(1191, pass);
    println!("Day 1 part 1 passed");
    Ok(())
}

struct Safe {
    dial: i16,
}

impl Safe {
    fn new() -> Self {
        Safe { dial: 50 }
    }

    fn rotate_left(&mut self) {
        self.dial -= 1;
        if self.dial == -1 {
            self.dial = 99;
        }
    }

    fn rotate_right(&mut self) {
        self.dial += 1;
        if self.dial == 100 {
            self.dial = 0;
        }
    }
}

fn parse_op(s: &str) -> (String, usize) {
    let (direction, n) = s.split_at(1);
    let amount = n.parse().expect("it's a number");

    (direction.to_string(), amount)
}
