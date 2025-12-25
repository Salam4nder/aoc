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
    let total = World::new(m).destroy();

    assert_eq!(8701, total);
    println!("Day 4 part 2 passed");
    Ok(())
}

struct World {
    inner: Vec<Vec<char>>,
    current_idx: (usize, usize),
    destroyed: u32,
    should_destroy: Vec<(usize, usize)>,
}

impl World {
    fn new(inner: Vec<Vec<char>>) -> Self {
        Self {
            inner,
            current_idx: (0, 0),
            destroyed: 0,
            should_destroy: vec![],
        }
    }

    fn total(&mut self) -> bool {
        let mut destroyed = false;
        for vector in &self.inner {
            for v in vector {
                if *v == '@' && self.scores() {
                    self.destroyed += 1;
                    self.should_destroy.push(self.current_idx);
                    destroyed = true;
                }
                self.current_idx.1 += 1;
            }
            self.current_idx.1 = 0;
            self.current_idx.0 += 1;
        }
        self.current_idx.0 = 0;
        destroyed
    }

    fn destroy(&mut self) -> u32 {
        while self.total() {
            for (o, i) in &self.should_destroy {
                let outer = self.inner.get_mut(*o).expect("it's there");
                *outer.get_mut(*i).expect("it's there") = '.';
            }
        }
        self.destroyed
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
