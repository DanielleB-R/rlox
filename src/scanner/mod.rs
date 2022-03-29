use std::str;

mod source;
mod token_type;

use source::Source;
pub use token_type::TokenType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub line: u32,
    pub slice: &'a str,
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

#[derive(Debug, Clone, Copy)]
pub struct Scanner<'a> {
    source: Source<'a>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: Source::new(source),
            line: 1,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            line: self.line,
            slice: self.source.current_str(),
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            token_type: TokenType::Error,
            line: self.line,
            slice: message,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.source.peek() {
                b' ' | b'\t' | b'\r' => {
                    self.source.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.source.advance();
                }
                b'/' => {
                    if self.source.peek_next() != b'/' {
                        return;
                    }
                    while self.source.peek() != b'\n' && !self.source.is_at_end() {
                        self.source.advance();
                    }
                }
                _ => break,
            }
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.source.reset();

        if self.source.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.source.advance();
        if is_alpha(c) {
            return self.identifier();
        }
        if c.is_ascii_digit() {
            return self.number();
        }

        match c {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b';' => self.make_token(TokenType::Semicolon),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),

            b'!' => {
                let token_type = if self.source.match_char(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token_type)
            }
            b'=' => {
                let token_type = if self.source.match_char(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token_type)
            }
            b'<' => {
                let token_type = if self.source.match_char(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token_type)
            }
            b'>' => {
                let token_type = if self.source.match_char(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token_type)
            }

            b'"' => self.string(),

            _ => self.error_token("Unexpected character"),
        }
    }

    fn string(&mut self) -> Token<'a> {
        while self.source.peek() != b'"' {
            if self.source.peek() == b'\n' {
                self.line += 1;
            }
            self.source.advance();
        }

        if self.source.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.source.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token<'a> {
        while self.source.peek().is_ascii_digit() {
            self.source.advance();
        }

        if self.source.peek() == b'.' && self.source.peek_next().is_ascii_digit() {
            self.source.advance();
            while self.source.peek().is_ascii_digit() {
                self.source.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'a> {
        while is_alpha(self.source.peek()) || self.source.peek().is_ascii_digit() {
            self.source.advance();
        }

        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        let identifier = self.source.current_str().as_bytes();
        match identifier[0] {
            b'a' => check_keyword(&identifier[1..], "nd", TokenType::And),
            b'c' => check_keyword(&identifier[1..], "lass", TokenType::Class),
            b'e' => check_keyword(&identifier[1..], "lse", TokenType::Else),
            b'i' => check_keyword(&identifier[1..], "f", TokenType::If),
            b'n' => check_keyword(&identifier[1..], "il", TokenType::Nil),
            b'o' => check_keyword(&identifier[1..], "r", TokenType::Or),
            b'p' => check_keyword(&identifier[1..], "rint", TokenType::Print),
            b'r' => check_keyword(&identifier[1..], "eturn", TokenType::Return),
            b's' => check_keyword(&identifier[1..], "uper", TokenType::Super),
            b'v' => check_keyword(&identifier[1..], "ar", TokenType::Var),
            b'w' => check_keyword(&identifier[1..], "hile", TokenType::While),

            b'f' => {
                if let Some(c2) = identifier.get(1) {
                    match c2 {
                        b'a' => check_keyword(&identifier[2..], "lse", TokenType::False),
                        b'o' => check_keyword(&identifier[2..], "r", TokenType::For),
                        b'u' => check_keyword(&identifier[2..], "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            b't' => {
                if let Some(c2) = identifier.get(1) {
                    match c2 {
                        b'h' => check_keyword(&identifier[2..], "is", TokenType::This),
                        b'r' => check_keyword(&identifier[2..], "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            _ => TokenType::Identifier,
        }
    }
}

fn check_keyword(identifier: &[u8], rest: &str, token_type: TokenType) -> TokenType {
    if identifier == rest.as_bytes() {
        token_type
    } else {
        TokenType::Identifier
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut scanner = Scanner::new("");

        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_pure_whitespace() {
        let mut scanner = Scanner::new(" \t\r\n");

        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_comment() {
        let mut scanner = Scanner::new("// this is a comment\n");

        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_single_token() {
        let mut scanner = Scanner::new("(");

        assert_eq!(scanner.scan_token().token_type, TokenType::LeftParen);
        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_double_token() {
        let mut scanner = Scanner::new("( )");

        assert_eq!(scanner.scan_token().token_type, TokenType::LeftParen);
        assert_eq!(scanner.scan_token().token_type, TokenType::RightParen);
        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_two_char_tokens() {
        let mut scanner = Scanner::new("= == != !");

        assert_eq!(scanner.scan_token().token_type, TokenType::Equal);
        assert_eq!(scanner.scan_token().token_type, TokenType::EqualEqual);
        assert_eq!(scanner.scan_token().token_type, TokenType::BangEqual);
        assert_eq!(scanner.scan_token().token_type, TokenType::Bang);
        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"abc\"");

        assert_eq!(
            scanner.scan_token(),
            Token {
                token_type: TokenType::String,
                line: 1,
                slice: "\"abc\"",
            }
        )
    }

    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("( 12.4 \n33 )");

        assert_eq!(scanner.scan_token().token_type, TokenType::LeftParen);
        assert_eq!(
            scanner.scan_token(),
            Token {
                token_type: TokenType::Number,
                line: 1,
                slice: "12.4"
            }
        );
        assert_eq!(
            scanner.scan_token(),
            Token {
                token_type: TokenType::Number,
                line: 2,
                slice: "33"
            }
        );
        assert_eq!(scanner.scan_token().token_type, TokenType::RightParen);
        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_one_identifier() {
        let mut scanner = Scanner::new("foo");

        assert_eq!(
            scanner.scan_token(),
            Token {
                token_type: TokenType::Identifier,
                line: 1,
                slice: "foo",
            }
        );
        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }

    #[test]
    fn test_keyword_identification() {
        let all_keywords =
            "and class else false for fun if nil or print return super this true var while";

        let mut scanner = Scanner::new(all_keywords);

        assert_eq!(scanner.scan_token().token_type, TokenType::And);
        assert_eq!(scanner.scan_token().token_type, TokenType::Class);
        assert_eq!(scanner.scan_token().token_type, TokenType::Else);
        assert_eq!(scanner.scan_token().token_type, TokenType::False);
        assert_eq!(scanner.scan_token().token_type, TokenType::For);
        assert_eq!(scanner.scan_token().token_type, TokenType::Fun);
        assert_eq!(scanner.scan_token().token_type, TokenType::If);
        assert_eq!(scanner.scan_token().token_type, TokenType::Nil);
        assert_eq!(scanner.scan_token().token_type, TokenType::Or);
        assert_eq!(scanner.scan_token().token_type, TokenType::Print);
        assert_eq!(scanner.scan_token().token_type, TokenType::Return);
        assert_eq!(scanner.scan_token().token_type, TokenType::Super);
        assert_eq!(scanner.scan_token().token_type, TokenType::This);
        assert_eq!(scanner.scan_token().token_type, TokenType::True);
        assert_eq!(scanner.scan_token().token_type, TokenType::Var);
        assert_eq!(scanner.scan_token().token_type, TokenType::While);
        assert_eq!(scanner.scan_token().token_type, TokenType::EOF);
    }
}
