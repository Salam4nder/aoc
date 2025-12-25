use crate::csv;
use crate::file;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day2.txt")?;
    let csv_reader = csv::Reader::new(reader);

    let mut password: u64 = 0;
    for v in csv_reader {
        let s = std::str::from_utf8(&v)?;
        let Some((from, to)) = s.split_once('-') else {
            return Err("splitting on -".into());
        };
        let from: u64 = from.parse()?;
        let to: u64 = to.parse()?;
        for val in from..=to {
            let digit_len = val.ilog10() + 1;
            let finder = FactorFinder::new(digit_len as usize);
            for f in finder {
                let mut broken = false;
                let divisor = 10u64.pow(f as u32);
                let cmp = val % divisor;
                let mut vv = val / divisor;
                for _ in 0..=digit_len {
                    if vv == 0 {
                        break;
                    }
                    let chunk = vv % divisor;
                    if chunk != cmp {
                        broken = true;
                        break;
                    }
                    vv /= divisor;
                }
                if !broken {
                    password += val as u64;
                    break;
                }
            }
        }
    }

    assert_eq!(34284458938, password);
    println!("Day 2 part 2 passed");
    Ok(())
}

struct FactorFinder {
    i: usize,
    target: usize,
}

impl FactorFinder {
    fn new(target: usize) -> Self {
        FactorFinder { i: 1, target }
    }
}

impl Iterator for FactorFinder {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.target {
            if self.target.is_multiple_of(self.i) {
                let tmp = self.i;
                self.i += 1;
                return Some(tmp);
            }
            self.i += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_iterator() {
        {
            let mut it = FactorFinder::new(6).into_iter();
            assert_eq!(Some(1), it.next());
            assert_eq!(Some(2), it.next());
            assert_eq!(Some(3), it.next());
        }

        {
            let mut it = FactorFinder::new(36).into_iter();
            assert_eq!(Some(1), it.next());
            assert_eq!(Some(2), it.next());
            assert_eq!(Some(3), it.next());
            assert_eq!(Some(4), it.next());
            assert_eq!(Some(6), it.next());
            assert_eq!(Some(9), it.next());
            assert_eq!(Some(12), it.next());
            assert_eq!(Some(18), it.next());
        }
    }
}
