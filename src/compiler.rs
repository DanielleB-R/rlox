use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);

    let mut line: u32 = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line
        } else {
            print!("   | ");
        }
        println!("{:2} '{}'", token.token_type as u8, token.slice);

        if token.token_type == TokenType::EOF {
            break;
        }
    }
}
