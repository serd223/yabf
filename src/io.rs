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
    pub fn with_source(self, f: fn() -> char) -> Self {
        Self {
            in_source: f,
            ..self
        }
    }

    pub fn with_flush(self, f: fn(&mut Self) -> Result<(), ()>) -> Self {
        Self { flush: f, ..self }
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
    static mut OUT_FLUSH: String = String::new();
    #[test]
    fn simple_io() {
        let mut io = BfIO::default().with_source(|| 'a').with_flush(|bfio| {
            while let Some(c) = bfio.pop_out() {
                unsafe { OUT_FLUSH.push(c) }
                // If BfIO's 'flush' field was a generic type that implements the FnMut trait, this unsafe mutable static usage wouldn't be necessary.
            }
            Ok(())
        });

        io.write_out('b');

        io.getc();
        assert_eq!(io.popc(), Some('a'));
        unsafe {
            assert_eq!(OUT_FLUSH, String::new());
        }
        assert_eq!(io.read_in(), 'a');
        unsafe {
            assert_eq!(OUT_FLUSH, String::from('b'));
        }

        io.write_out('a');
        assert_eq!(io.pop_out(), Some('a'));

        io.write_out('a');
        io.flush().unwrap();
        unsafe {
            assert_eq!(OUT_FLUSH, String::from("ba"));
        }
    }
}
