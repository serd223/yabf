/// A structure for organizing a BfInstance's IO operations.
pub struct BfIO {
    pub out_buf: Vec<char>,
    in_buf: Vec<char>,
}

impl Default for BfIO {
    fn default() -> Self {
        Self {
            out_buf: vec![],
            in_buf: vec![],
        }
    }
}

impl BfIO {
    pub fn getc<SOURCE>(&mut self, mut input_source: SOURCE)
    where
        SOURCE: FnMut() -> char,
    {
        self.in_buf.insert(0, (input_source)());
    }

    pub fn popc(&mut self) -> Option<char> {
        self.in_buf.pop()
    }

    pub fn flush<FLUSH>(&mut self, mut flush: FLUSH) -> Result<(), ()>
    where
        FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
    {
        (flush)(&mut self.out_buf)
    }

    pub fn read_in<SOURCE, FLUSH>(&mut self, input_source: SOURCE, flush: FLUSH) -> char
    where
        SOURCE: FnMut() -> char,
        FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
    {
        self.flush(flush).expect("Couldn't flush Out Buffer");
        self.getc(input_source);
        self.popc().expect("In Buffer is empty")
    }

    pub fn write_out(&mut self, c: char) {
        self.out_buf.insert(0, c);
    }

    pub fn pop_out(&mut self) -> Option<char> {
        self.out_buf.pop()
    }
}

#[cfg(test)]
mod tests {

    use super::BfIO;
    #[test]
    fn simple_io() {
        let mut out_flush = String::new();
        let input_source = || 'a';
        let mut io = BfIO::default();

        io.write_out('b');

        io.getc(&input_source);
        assert_eq!(io.popc(), Some('a'));
        assert_eq!(out_flush.as_str(), "");

        assert_eq!(
            io.read_in(&input_source, |out: &mut Vec<char>| {
                while let Some(c) = out.pop() {
                    out_flush.push(c);
                }
                Ok(())
            }),
            'a'
        );
        assert_eq!(out_flush.as_str(), "b");

        io.write_out('a');
        assert_eq!(io.pop_out(), Some('a'));

        io.write_out('a');
        io.flush(|out: &mut Vec<char>| {
            while let Some(c) = out.pop() {
                out_flush.push(c);
            }
            Ok(())
        })
        .unwrap();
        assert_eq!(out_flush.as_str(), "ba");
    }
}
