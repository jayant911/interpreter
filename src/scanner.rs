#![allow(dead_code, unused_variables)]

use std::mem::take;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }
        Ok(take(&mut self.tokens))
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParn),
            ')' => self.add_token(TokenType::RightParn),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Start),
            '!' => {
                let token = if self.match_for_multichar('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token)
            }
            '=' => {
                let token = if self.match_for_multichar('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token)
            }
            '<' => {
                let token = if self.match_for_multichar('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token)
            }
            '>' => {
                let token = if self.match_for_multichar('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token)
            }
            '/' => {
                if self.match_for_multichar('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => Ok(()), // Ignore whitespace
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '"' => self.string_literal(),
            c => {
                if c.is_ascii_digit() {
                    self.number()
                } else {
                    Err(format!(
                        "Unrecognized character: {} at line {}",
                        c, self.line
                    ))
                }
            }
        }
    }

    fn match_for_multichar(&mut self, expected: char) -> bool {
        match self.source.chars().nth(self.current as usize) {
            Some(c) => {
                if c == expected {
                    self.current += 1;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), String> {
        self.add_token_literal(token_type, None)
    }

    fn add_token_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<LiteralValue>,
    ) -> Result<(), String> {
        let text = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
        Ok(())
    }

    fn string_literal(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start as usize + 1..self.current as usize - 1].to_string();
        self.add_token_literal(TokenType::String, Some(LiteralValue::String(value)))
    }

    fn number(&mut self) -> Result<(), String> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        match self.source[self.start as usize..self.current as usize]
            .to_string()
            .parse::<f64>()
        {
            Ok(number) => {
                self.add_token_literal(TokenType::Number, Some(LiteralValue::Float(number)))
            }
            Err(_) => Err("Unknown digit parsing".to_string()),
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current as usize + 1 >= self.source.len() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current as usize + 1)
            .unwrap_or('\0')
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

    pub fn as_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_one_char_token() {
        let code = "{()} ,.;/ //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::LeftParn);
        assert_eq!(tokens[2].token_type, TokenType::RightParn);
        assert_eq!(tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::Dot);
        assert_eq!(tokens[6].token_type, TokenType::Semicolon);
        assert_eq!(tokens[7].token_type, TokenType::Slash);
        assert_eq!(tokens[8].token_type, TokenType::Eof);
    }

    #[test]
    fn test_hanele_two_cahr_token() {
        let code = "{()}  <= != == ,.;/ //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::LeftParn);
        assert_eq!(tokens[2].token_type, TokenType::RightParn);
        assert_eq!(tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(tokens[4].token_type, TokenType::LessEqual);
        assert_eq!(tokens[5].token_type, TokenType::BangEqual);
        assert_eq!(tokens[6].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[7].token_type, TokenType::Comma);
        assert_eq!(tokens[8].token_type, TokenType::Dot);
        assert_eq!(tokens[9].token_type, TokenType::Semicolon);
        assert_eq!(tokens[10].token_type, TokenType::Slash);
        assert_eq!(tokens[11].token_type, TokenType::Eof);
    }

    #[test]
    fn test_string_literal() {
        let code = "/ \"this is string <> =!\" //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(
            tokens[1].literal,
            Some(LiteralValue::String("this is string <> =!".to_string()))
        );
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_string_literal_unterminated() {
        let code = "\"this is string";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens();
        match tokens {
            Err(msg) => (),
            Ok(_) => panic!("should have failed"),
        }
    }

    #[test]
    fn test_string_literal_multiline() {
        let code = "\"this is string \n with multiline\"";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(
            tokens[0].literal,
            Some(LiteralValue::String(
                "this is string \n with multiline".to_string()
            ))
        );
    }

    #[test]
    fn test_number_literal() {
        let code = "/84 //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].literal, Some(LiteralValue::Float(84.0)));
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_number_literal_with_point() {
        let code = "/84.32 //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].literal, Some(LiteralValue::Float(84.32)));
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_number_literal_with_front_point() {
        let code = "/.84 //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(tokens[2].literal, Some(LiteralValue::Float(84.0)));
        assert_eq!(tokens[2].token_type, TokenType::Number);
    }

    #[test]
    fn test_number_literal_endwith_dot() {
        let code = "/78. //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].literal, Some(LiteralValue::Float(78.0)));
        assert_eq!(tokens[2].token_type, TokenType::Dot);
    }

    #[test]
    fn test_number_literal_multiline() {
        let code = "/78\n89 //";
        let mut scanner = Scanner::new(code);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].literal, Some(LiteralValue::Float(78.0)));
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].literal, Some(LiteralValue::Float(89.0)));
    }
}
