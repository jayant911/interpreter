use core::fmt;
use std::borrow::Cow;

use miette::{Error, LabeledSpan, WrapErr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'de> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    Equal,
    Ident(&'de str),
    Number(&'de str, f64),
    String(&'de str),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::EqualEqual => write!(f, "EQIAL_EQUAL == null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::Less => write!(f, "LESS < null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::Bang => write!(f, "BANG ! null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::String(s) => write!(f, "STRING \"{s}\" {}", Token::unescape(s)),
            Token::Ident(i) => write!(f, "IDENT  {i}"),
            Token::Number(ltr, n) => write!(f, "NUMBER {ltr} {n}"),
            Token::And => write!(f, "AND And null"),
            Token::Class => write!(f, "CLASS Class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::False => write!(f, "FALSE false null"),
            Token::For => write!(f, "FOR for null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::If => write!(f, "IF if null"),
            Token::Nil => write!(f, "NIL nil null"),
            Token::Or => write!(f, "OR or null"),
            Token::Print => write!(f, "PRINT Print null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::True => write!(f, "TRUE true null"),
            Token::Var => write!(f, "VAR var null"),
            Token::While => write!(f, "WHILE while null"),
        }
    }
}

impl Token<'_> {
    pub fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
        todo!()
    }
}

pub struct Lexer<'de> {
    whole: &'de str,
    rest: &'de str,
    byte: usize, // current charecter
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;

    /// Once the iterator returns `Err`, it will only return `None`.
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.rest = chars.as_str();
        self.byte += c.len_utf8();

        enum Started {
            String,
            Number,
            Ident,
            Less,
            Greater,
            Bang,
            Equal,
        }

        let started = match c {
            '(' => return Some(Ok(Token::LeftParen)),
            ')' => return Some(Ok(Token::RightParen)),
            '{' => return Some(Ok(Token::LeftBrace)),
            '}' => return Some(Ok(Token::RightBrace)),
            ',' => return Some(Ok(Token::Comma)),
            '.' => return Some(Ok(Token::Dot)),
            '-' => return Some(Ok(Token::Minus)),
            '+' => return Some(Ok(Token::Plus)),
            ';' => return Some(Ok(Token::Semicolon)),
            '*' => return Some(Ok(Token::Star)),
            '/' => return Some(Ok(Token::Slash)),
            '<' => Started::Less,
            '>' => Started::Greater,
            '!' => Started::Bang,
            '=' => Started::Equal,
            '"' => Started::String,
            '0'..='9' => Started::Number,
            'a'..='z' | '_' => Started::Ident,
            c => {
                return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.byte - c.len_utf8()..self.byte, "this charecter"),
                    ],
                    "Unexpected token '{c}' in input",
                }
                .with_source_code(self.whole.to_string())));
            }
        };

        match started {
            Started::String => todo!(),
            Started::Number => todo!(),
            Started::Ident => todo!(),
            Started::Less => {
                if self.rest.starts_with('=') {
                    self.rest = &self.rest[1..];
                    self.byte += 1;
                    return Some(Ok(Token::LessEqual));
                } else {
                    return Some(Ok(Token::Less));
                }
            }
            Started::Greater => {
                if self.rest.starts_with('=') {
                    self.rest = &self.rest[1..];
                    self.byte += 1;
                    return Some(Ok(Token::GreaterEqual));
                } else {
                    return Some(Ok(Token::Greater));
                }
            }
            Started::Bang => {
                if self.rest.starts_with('=') {
                    self.rest = &self.rest[1..];
                    self.byte += 1;
                    return Some(Ok(Token::BangEqual));
                } else {
                    return Some(Ok(Token::Bang));
                }
            }
            Started::Equal => {
                if self.rest.starts_with('=') {
                    self.rest = &self.rest[1..];
                    self.byte += 1;
                    return Some(Ok(Token::EqualEqual));
                } else {
                    return Some(Ok(Token::Equal));
                }
            }
        }
    }
}
