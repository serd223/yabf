/// A structure for organizing a BfInstance's IO operations.
pub struct BfIO {
    pub out_buf: Vec<char>,
    in_buf: Vec<char>,
    in_source: fn() -> char,
    flush: fn(&mut BfIO) -> Result<(), ()>,
}

impl Default for BfIO {
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
            flush: |io| {
                while let Some(c) = io.pop_out() {
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

impl BfIO {
    pub fn with_source(f: fn() -> char) -> Self {
        Self {
            in_source: f,
            ..Default::default()
        }
    }

    pub fn getc(&mut self) {
        self.in_buf.insert(0, (self.in_source)());
    }

    pub fn popc(&mut self) -> Option<char> {
        self.in_buf.pop()
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        (self.flush)(self)
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
    use super::BfIO;
    #[test]
    fn simple_usage() {
        let mut io = BfIO::with_source(|| 'a');

        io.getc();
        assert_eq!(io.popc(), Some('a'));

        assert_eq!(io.read_in(), 'a');

        io.write_out('a');
        assert_eq!(io.pop_out(), Some('a'));
    }
}
