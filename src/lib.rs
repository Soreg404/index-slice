pub struct IndexSlice {
    from: usize,
    to: usize
}

impl IndexSlice {
    pub fn new(from: usize, to: usize) -> Self {
        assert!(from <= to);
        Self {
            from,
            to
        }
    }
    pub fn as_slice_of<'a>(&self, target: &'a [u8]) -> &'a [u8] {
        &target[self.from..self.to]
    }
    pub fn from(&self) -> usize {
        self.from
    }
    pub fn to(&self) -> usize {
        self.from
    }
    pub fn offset_forward(self, offset: usize) -> Self {
        Self::new(self.from + offset, self.to + offset)
    }
    pub fn offset_backward(self, offset: usize) -> Self {
        Self::new(self.from - offset, self.to - offset)
    }
    pub fn window(&self, range: std::ops::Range<usize>) -> Self {
        Self::new(self.from + range.start, self.to - range.end)
    }
}

#[cfg(test)]
mod tests {
    use super::IndexSlice;

    #[test]
    fn simple_test() {
        let buffer = b"hello world!";
        let w_hello = IndexSlice::new(0, 5);
        let w_world = IndexSlice::new(6, buffer.len());
        println!(
            "1st word: {:?}\n2nd word: {:?}",
            String::from_utf8_lossy(w_hello.as_slice_of(buffer)),
            String::from_utf8_lossy(w_world.as_slice_of(buffer)),
        );
    }

}
