use std::io::BufRead;

pub struct Reader<B> {
    buf: B,
}

impl<B: BufRead> Reader<B> {
    pub fn new(buf: B) -> Self {
        Self { buf }
    }
}

impl<B: BufRead> Iterator for Reader<B> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = vec![];
        let delim = b',';

        match self.buf.read_until(delim, &mut v) {
            Ok(0) => None,
            Ok(_) => {
                v.retain(|b| !b.is_ascii_whitespace() && *b != delim);
                Some(v)
            }
            Err(_) => None,
        }
    }
}
