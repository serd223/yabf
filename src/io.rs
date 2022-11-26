/// A structure for organizing a BfInstance's IO operations.
pub struct BfIO {
    out_buf: Vec<char>,
    in_buf: Vec<char>,
    in_source: fn() -> char
}

impl Default for BfIO {
    fn default() -> Self {
        Self {
            out_buf: vec![],
            in_buf: vec![],
            in_source: || {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).expect("Couldn't read user input.");
                let c: char = s.chars().nth(0).unwrap();
                c
            }
        }
    }
}

impl BfIO {

    pub fn getc(&mut self) {
        self.in_buf.insert(0, (self.in_source)());
    }

    pub fn popc(&mut self) -> Option<char> {
        self.in_buf.pop()
    }

    pub fn read_in(&mut self) -> char {
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