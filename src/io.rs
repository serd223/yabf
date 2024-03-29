/// A structure for organizing a BfInstance's IO operations.
#[derive(Default)]
pub struct BfIO {
    pub out_buf: Vec<char>,
    pub in_buf: Vec<char>,
}

impl BfIO {
    /// Tries to get a single character from `input_source`.
    /// If that character is not None, inserts that character to the start of `in_buf`.
    pub fn getc<SOURCE>(&mut self, mut input_source: SOURCE)
    where
        SOURCE: FnMut() -> Option<char>,
    {
        let res = (input_source)();
        match res {
            None => (),
            Some(c) => self.in_buf.insert(0, c),
        }
    }

    /// 'Pop's the oldest character from `in_buf`
    pub fn popc(&mut self) -> Option<char> {
        self.in_buf.pop()
    }

    /// Executes the provided `flush` function and supplies `out_buf` as the arguement.
    pub fn flush<FLUSH>(&mut self, mut flush: FLUSH) -> Result<(), ()>
    where
        FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
    {
        (flush)(&mut self.out_buf)
    }

    /// Simple convenience function that `flush`es `out_buf` and then returns the result of `input_source`
    pub fn read_in<SOURCE, FLUSH>(&mut self, input_source: SOURCE, flush: FLUSH) -> Option<char>
    where
        SOURCE: FnMut() -> Option<char>,
        FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
    {
        self.flush(flush).expect("Couldn't flush Out Buffer");
        self.getc(input_source);
        self.popc()
    }

    /// Writes a single character to the start of `out_buf`
    pub fn write_out(&mut self, c: char) {
        self.out_buf.insert(0, c);
    }

    /// 'Pop's the oldest character from `out_buf`
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
        let input_source = || Some('a');
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
            Some('a')
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
