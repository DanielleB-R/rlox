use std::str;

#[derive(Debug, Clone, Copy)]
pub struct Source<'a> {
    start: usize,
    current: usize,
    source: &'a [u8],
}

impl<'a> Source<'a> {
    pub fn new(source: &'a str) -> Self {
        assert!(source.is_ascii());
        Self {
            start: 0,
            current: 0,
            source: source.as_bytes(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    pub fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    pub fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    pub fn peek(&self) -> u8 {
        self.source.get(self.current).copied().unwrap_or_default()
    }

    pub fn peek_next(&self) -> u8 {
        self.source
            .get(self.current + 1)
            .copied()
            .unwrap_or_default()
    }

    pub fn current_str(&self) -> &'a str {
        // This is guaranteed safe by the constructor assertion
        unsafe { str::from_utf8_unchecked(&self.source[self.start..self.current]) }
    }

    pub fn reset(&mut self) {
        self.start = self.current;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_source_advancement() {
        let mut src = Source::new("abcd");

        assert_eq!(src.current_str(), "");
        assert!(!src.is_at_end());

        src.advance();
        assert!(!src.is_at_end());
        assert_eq!(src.peek(), b'b');
        assert_eq!(src.peek_next(), b'c');
        assert_eq!(src.current_str(), "a");

        src.advance();
        assert!(!src.is_at_end());
        assert_eq!(src.current_str(), "ab");

        src.reset();
        assert!(!src.is_at_end());
        assert_eq!(src.current_str(), "");
        assert_eq!(src.peek(), b'c');
        assert_eq!(src.peek_next(), b'd');

        src.advance();
        assert!(!src.is_at_end());
        assert_eq!(src.current_str(), "c");
        assert_eq!(src.peek(), b'd');
        assert_eq!(src.peek_next(), 0);

        src.advance();
        assert!(src.is_at_end());
        assert_eq!(src.current_str(), "cd");
        assert_eq!(src.peek(), 0);
        assert_eq!(src.peek_next(), 0);
    }

    #[test]
    fn test_match_char() {
        let mut source = Source::new("12.5");

        assert_eq!(source.current_str(), "");
        assert!(!source.match_char(b'5'));
        assert_eq!(source.current_str(), "");

        assert!(source.match_char(b'1'));
        assert_eq!(source.current_str(), "1");

        source.advance();
        source.advance();
        source.advance();

        assert!(source.is_at_end());
        assert!(!source.match_char(b'6'));
    }
}
