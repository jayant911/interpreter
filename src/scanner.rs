#![allow(dead_code)]

#[derive(Debug)]
pub struct Scanner {}

impl Scanner {
    pub fn new(_source: &str) -> Self {
        Self {}
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
        //TODO:
        todo!()
    }
}

#[derive(Debug)]
pub enum TokenType {
    //Single char token
    LeftParn,
    RightParn,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Start,

    // One or tow cahrs
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Ientifire,
    String,
    Number,

    // Keywords
    Aand,
    Class,
    False,
    Fun,
    For,
    IfNil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Identifire(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: u64,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: u64,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }
}
