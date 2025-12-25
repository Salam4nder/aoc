struct Iterator {
    n: u128,
    idx: usize,
}

impl Iterator {
    fn new(n: u128) -> Self {
        Self { n, idx: 0 }
    }

    /// # Panics
    /// 16-bit architecture not supported.
    fn len(&self) -> usize {
        usize::try_from(self.n.ilog10() + 1).expect("not running on 16-bit architecture")
    }
}

impl Iterator for DigitIterator {
    type Item = (usize, u8);
    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let num_of_zeroes = self.n.ilog10();
            let divisor = 10u32.pow(num_of_zeroes);
            let tmp_val = self.n / u128::from(divisor);
            let tmp_idx = self.idx;
            self.idx += 1;
            self.n %= u128::from(divisor);
            let val = u8::try_from(tmp_val).expect("One digit fits in u8");
            return Some((tmp_idx, val));
        }
        None
    }
}
