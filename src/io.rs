/// A structure for organizing a BfInstance's IO operations.
pub struct BfIO<SOURCE, FLUSH>
where
    SOURCE: FnMut() -> char,
    FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
{
    pub out_buf: Vec<char>,
    in_buf: Vec<char>,
    in_source: SOURCE,
    flush: FLUSH,
}

impl Default for BfIO<fn() -> char, fn(&mut Vec<char>) -> Result<(), ()>> {
    fn default() -> Self {
        Self {
            out_buf: vec![],
            in_buf: vec![],
            in_source: || {
                let mut s = String::new();
                std::io::stdin()
                    .read_line(&mut s)
                    .expect("Couldn't read user input.");
                let c: char = s.chars().nth(0).unwrap();
                c
            },
            flush: |out| {
                while let Some(c) = out.pop() {
                    print!("{c}")
                }
                use std::io::Write;
                match std::io::stdout().flush() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            },
        }
    }
}

impl<SOURCE, FLUSH> BfIO<SOURCE, FLUSH>
where
    SOURCE: FnMut() -> char,
    FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
{
    pub fn new(source: SOURCE, flush: FLUSH) -> Self {
        Self {
            in_source: source,
            flush: flush,
            in_buf: vec![],
            out_buf: vec![],
        }
    }

    pub fn getc(&mut self) {
        self.in_buf.insert(0, (self.in_source)());
    }

    pub fn popc(&mut self) -> Option<char> {
        self.in_buf.pop()
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        (self.flush)(&mut self.out_buf)
    }

    pub fn read_in(&mut self) -> char {
        self.flush().expect("Couldn't flush Out Buffer");
        self.getc();
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
    use std::cell::RefCell;

    use super::BfIO;
    #[test]
    fn simple_io() {
        let out_flush = RefCell::new(String::new());
        let mut io = BfIO::new(
            || 'a',
            |out| {
                while let Some(c) = out.pop() {
                    out_flush.borrow_mut().push(c);
                }
                Ok(())
            },
        );

        io.write_out('b');

        io.getc();
        assert_eq!(io.popc(), Some('a'));
        assert_eq!(out_flush.borrow().as_str(), "");

        assert_eq!(io.read_in(), 'a');
        assert_eq!(out_flush.borrow().as_str(), "b");

        io.write_out('a');
        assert_eq!(io.pop_out(), Some('a'));

        io.write_out('a');
        io.flush().unwrap();
        assert_eq!(out_flush.borrow().as_str(), "ba");
    }
}
