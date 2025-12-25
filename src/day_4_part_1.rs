use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day4.txt")?;
    let mut m: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let mut inner: Vec<char> = vec![];
        let line = line?;
        let line: Vec<char> = line.chars().collect();
        for c in line {
            inner.push(c);
        }
        m.push(inner);
    }
    let total = World::new(m).total();

    assert_eq!(1451, total);
    println!("Day 4 part 1 passed");
    Ok(())
}

struct World {
    inner: Vec<Vec<char>>,
    current_idx: (usize, usize),
}

impl World {
    fn new(inner: Vec<Vec<char>>) -> Self {
        Self {
            inner,
            current_idx: (0, 0),
        }
    }

    fn total(&mut self) -> u32 {
        let mut total: u32 = 0;
        for vector in &self.inner {
            for v in vector {
                if *v == '@' && self.scores() {
                    total += 1;
                }
                self.current_idx.1 += 1;
            }
            self.current_idx.1 = 0;
            self.current_idx.0 += 1;
        }
        total
    }

    fn scores(&self) -> bool {
        self.left()
            + self.right()
            + self.up()
            + self.down()
            + self.up_right()
            + self.up_left()
            + self.down_left()
            + self.down_right()
            < 4
    }

    fn left(&self) -> u8 {
        if self.current_idx.1 == 0 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0)
            && let Some(v) = out.get(self.current_idx.1 - 1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn right(&self) -> u8 {
        if self.current_idx.1 == 139 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0)
            && let Some(v) = out.get(self.current_idx.1 + 1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn up(&self) -> u8 {
        if self.current_idx.0 == 0 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0 - 1)
            && let Some(v) = out.get(self.current_idx.1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn down(&self) -> u8 {
        if self.current_idx.0 == 139 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0 + 1)
            && let Some(v) = out.get(self.current_idx.1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn up_right(&self) -> u8 {
        if self.current_idx.0 == 0 {
            return 0;
        }
        if self.current_idx.1 == 139 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0 - 1)
            && let Some(v) = out.get(self.current_idx.1 + 1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn up_left(&self) -> u8 {
        if self.current_idx.0 == 0 {
            return 0;
        }
        if self.current_idx.1 == 0 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0 - 1)
            && let Some(v) = out.get(self.current_idx.1 - 1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn down_right(&self) -> u8 {
        if self.current_idx.0 == 139 {
            return 0;
        }
        if self.current_idx.1 == 139 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0 + 1)
            && let Some(v) = out.get(self.current_idx.1 + 1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }

    fn down_left(&self) -> u8 {
        if self.current_idx.0 == 139 {
            return 0;
        }
        if self.current_idx.1 == 0 {
            return 0;
        }
        if let Some(out) = self.inner.get(self.current_idx.0 + 1)
            && let Some(v) = out.get(self.current_idx.1 - 1)
        {
            if *v == '@' { 1 } else { 0 }
        } else {
            0
        }
    }
}
