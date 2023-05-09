use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
#[repr(u8)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Default for Precedence {
    fn default() -> Self {
        Self::None
    }
}

impl Precedence {
    fn incr(self) -> Self {
        if self == Self::Primary {
            panic!("Can't increment primary precedence")
        }
        unsafe { mem::transmute(self as u8 + 1) }
    }
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            current: Default::default(),
            previous: Default::default(),
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();

            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.slice);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        let location = if token.token_type == TokenType::EOF {
            " at end".to_owned()
        } else if token.token_type == TokenType::Error {
            "".to_owned()
        } else {
            format!(" at {}", token.slice)
        };

        eprintln!("[line {}] Error{}: {}", token.line, location, message);

        self.had_error = true;
    }
}

macro_rules! rule_lookups {
    ($($token_type:ident, $prefix:ident, $infix:ident, $precedence:ident);+) =>{
        fn prefix_parser(&mut self, token_type: TokenType) -> bool {
            match token_type {
                $(TokenType::$token_type => { self.$prefix(); true }),+,
                _ => {self.parser.error("Expect expression."); false}
            }
        }

        fn infix_parser(&mut self, token_type: TokenType) {
            match token_type {
                $(TokenType::$token_type => self.$infix()),+,
                _ => {}
            }
        }

        fn precedence_for(&self, token_type: TokenType) -> Precedence {
            match token_type {
                $(TokenType::$token_type => Precedence::$precedence),+,
                _ => Precedence::None,
            }
        }
    }
}

macro_rules! opcode {
    ($self:ident, $opcode:ident) => {
        $self.emit_byte(OpCode::$opcode as u8)
    };
    ($self:ident, $opcode1:ident, $opcode2:ident) => {
        $self.emit_bytes(OpCode::$opcode1 as u8, OpCode::$opcode2 as u8)
    };
}

struct Compiler<'a> {
    parser: Parser<'a>,
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    fn new(parser: Parser<'a>, chunk: &'a mut Chunk) -> Self {
        Self { parser, chunk }
    }

    rule_lookups! {
        LeftParen, grouping, noop, None;
        Minus, unary, binary, Term;
        Plus, boom, binary, Term;
        Slash, boom, binary, Factor;
        Star, boom, binary, Factor;
        Bang, unary, noop, None;
        BangEqual, boom, binary, Equality;
        EqualEqual, boom, binary, Equality;
        Greater, boom, binary, Comparison;
        GreaterEqual, boom, binary, Comparison;
        Less, boom, binary, Comparison;
        LessEqual, boom, binary, Comparison;
        Number, number, noop, None;
        False, literal, noop, None;
        Nil, literal, noop, None;
        True, literal, noop, None
    }

    fn noop(&mut self) {}

    fn boom(&mut self) {
        panic!("boom");
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn number(&mut self) {
        self.emit_constant(self.parser.previous.slice.parse::<f64>().unwrap().into())
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant)
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        match u8::try_from(constant) {
            Err(_) => {
                self.parser.error("Too many constants in one chunk.");
                0
            }
            Ok(byte) => byte,
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::Not as u8),
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        self.parse_precedence(self.precedence_for(operator_type).incr());

        match operator_type {
            TokenType::BangEqual => opcode!(self, Equal, Not),
            TokenType::EqualEqual => opcode!(self, Equal),
            TokenType::Greater => opcode!(self, Greater),
            TokenType::GreaterEqual => opcode!(self, Less, Not),
            TokenType::Less => opcode!(self, Less),
            TokenType::LessEqual => opcode!(self, Greater, Not),
            TokenType::Plus => opcode!(self, Add),
            TokenType::Minus => opcode!(self, Subtract),
            TokenType::Star => opcode!(self, Multiply),
            TokenType::Slash => opcode!(self, Divide),
            _ => unreachable!(),
        };
    }

    fn literal(&mut self) {
        self.emit_byte(match self.parser.previous.token_type {
            TokenType::False => OpCode::False,
            TokenType::Nil => OpCode::Nil,
            TokenType::True => OpCode::True,
            _ => unreachable!(),
        } as u8)
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance();

        if !self.prefix_parser(self.parser.previous.token_type) {
            return;
        }

        while precedence <= self.precedence_for(self.parser.current.token_type) {
            self.parser.advance();
            self.infix_parser(self.parser.previous.token_type)
        }
    }
}

pub fn compile(source: &str, chunk: &mut Chunk) -> bool {
    let scanner = Scanner::new(source);
    let parser = Parser::new(scanner);
    let mut compiler = Compiler::new(parser, chunk);

    compiler.parser.advance();
    compiler.expression();
    compiler
        .parser
        .consume(TokenType::EOF, "Expect end of expression");

    compiler.end_compiler();
    !compiler.parser.had_error
}
