#[derive(Debug)]

pub struct BufReader<'a> {
    buf: &'a [u8],
    pub pos: usize,
}

impl<'a> BufReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn read(&mut self, len: usize) -> &[u8] {
        let buf = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        buf
    }

    pub fn goto(&mut self, pos: usize) -> () {
        self.pos = pos;
    }
}
