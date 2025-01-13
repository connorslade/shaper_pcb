use anyhow::{bail, Ok, Result};

pub struct Parser {
    pub chars: Vec<char>,
    pub idx: usize,
}

impl Parser {
    pub fn new(string: &str) -> Self {
        Self {
            chars: string.chars().collect(),
            idx: 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.idx >= self.chars.len()
    }

    pub fn next(&mut self) -> char {
        let out = self.chars[self.idx];
        self.idx += 1;

        out
    }

    pub fn take_while(&mut self, predacate: fn(char) -> bool) {
        while predacate(self.peek()) {
            self.next();
        }
    }

    pub fn peek(&self) -> char {
        self.chars[self.idx]
    }

    pub fn expect(&mut self, expected: &str) -> Result<()> {
        for char in expected.chars() {
            if self.is_eof() {
                bail!("Unexpected EOF");
            }

            if self.next() != char {
                bail!("Character mismatch");
            }
        }

        Ok(())
    }

    pub fn parse_int(&mut self) -> Result<u32> {
        let start = self.idx;

        self.take_while(|c| c.is_ascii_digit());
        let string = self.chars[start..self.idx].iter().collect::<String>();

        Ok(string.parse()?)
    }

    pub fn parse_float(&mut self) -> Result<f64> {
        let start = self.idx;

        self.take_while(|c| matches!(c, '0'..='9' | '.' | '-'));
        let string = self.chars[start..self.idx].iter().collect::<String>();
        // ↑ There has to be a way without allocating...

        Ok(string.parse()?)
    }

    pub fn next_line(&mut self) {
        while self.next() != '\n' && !self.is_eof() {}
    }
}
