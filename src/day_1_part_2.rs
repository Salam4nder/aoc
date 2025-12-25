use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day1.txt")?;
    let mut safe = Safe::new();
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
    }
    assert_eq!(6858, safe.counter);
    println!("Day 1 part 2 passed");
    Ok(())
}

struct Safe {
    dial: i16,
    counter: u32,
}

impl Safe {
    fn new() -> Self {
        Safe {
            dial: 50,
            counter: 0,
        }
    }

    fn rotate_left(&mut self) {
        self.dial -= 1;
        if self.dial == 0 {
            self.counter += 1;
        }
        if self.dial == -1 {
            self.dial = 99;
        }
    }

    fn rotate_right(&mut self) {
        self.dial += 1;
        if self.dial == 100 {
            self.dial = 0;
            self.counter += 1;
        }
    }
}

fn parse_op(s: &str) -> (String, usize) {
    let (direction, n) = s.split_at(1);
    let amount = n.parse().expect("it's a number");

    (direction.to_string(), amount)
}
