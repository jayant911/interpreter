#![allow(dead_code, unused_variables, unused_imports)]

use std::fmt;

use crate::scanner::{Token, TokenType};

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::StringValue(s) => write!(f, "{}", s),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

pub enum Expr {
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", left, operator.as_string(), right)
            }
            Expr::Unary { operator, right } => {
                write!(f, "{} {}", operator.as_string(), right)
            }
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Literal { value } => write!(f, "({})", value),
        }
    }
}

impl Expr {
    pub fn print(&self) {
        println!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_prent_ast() {
        // -(1 + 4) + (5 * 4 + 6) != (1 * 3)

        let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, 0);
        let plus_token = Token::new(TokenType::Plus, "+".to_string(), None, 0);
        let star_token = Token::new(TokenType::Start, "*".to_string(), None, 0);
        let bang_equal_token = Token::new(TokenType::BangEqual, "!=".to_string(), None, 0);

        // (1 + 4)
        let one_plus_four = Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                }),
                operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(4.0),
                }),
            }),
        };

        // -(1 + 4)
        let neg_one_plus_four = Expr::Unary {
            operator: minus_token,
            right: Box::new(one_plus_four),
        };

        // (5 * 4)
        let five_times_four = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(5.0),
            }),
            operator: star_token,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(4.0),
            }),
        };

        // (5 * 4 + 6)
        let five_times_four_plus_six = Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(five_times_four),
                operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(6.0),
                }),
            }),
        };

        // -(1 + 4) + (5 * 4 + 6)
        let left_side = Expr::Binary {
            left: Box::new(neg_one_plus_four),
            operator: plus_token,
            right: Box::new(five_times_four_plus_six),
        };

        // (1 * 3)
        let one_times_three = Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                }),
                operator: Token::new(TokenType::Start, "*".to_string(), None, 0),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(3.0),
                }),
            }),
        };

        // -(1 + 4) + (5 * 4 + 6) != (1 * 3)
        let ast = Expr::Binary {
            left: Box::new(left_side),
            operator: bang_equal_token,
            right: Box::new(one_times_three),
        };

        let result = format!("{}", ast);
        // -(1 + 4) + (5 * 4 + 6) != (1 * 3)
        assert_eq!(
            result,
            "((Minus - None (group ((1) Plus + None (4))) Plus + None (group (((5) Start * None (4)) Plus + None (6)))) BangEqual != None (group ((1) Start * None (3))))"
        );
    }

    #[test]
    fn literal_number() {
        let expr = Expr::Literal {
            value: LiteralValue::Number(42.0),
        };
        assert_eq!(format!("{}", expr), "(42)");
    }

    #[test]
    fn literal_string() {
        let expr = Expr::Literal {
            value: LiteralValue::StringValue("hello".to_string()),
        };
        assert_eq!(format!("{}", expr), "(hello)");
    }

    #[test]
    fn literal_bool_and_nil() {
        assert_eq!(
            format!(
                "{}",
                Expr::Literal {
                    value: LiteralValue::True
                }
            ),
            "(true)"
        );
        assert_eq!(
            format!(
                "{}",
                Expr::Literal {
                    value: LiteralValue::False
                }
            ),
            "(false)"
        );
        assert_eq!(
            format!(
                "{}",
                Expr::Literal {
                    value: LiteralValue::Nil
                }
            ),
            "(nil)"
        );
    }

    #[test]
    fn unary_negation() {
        // -5
        let expr = Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(5.0),
            }),
        };
        assert_eq!(format!("{}", expr), "Minus - None (5)");
    }

    #[test]
    fn simple_binary() {
        // 2 + 3
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(2.0),
            }),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(3.0),
            }),
        };
        assert_eq!(format!("{}", expr), "((2) Plus + None (3))");
    }

    #[test]
    fn grouping() {
        // (7)
        let expr = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(7.0),
            }),
        };
        assert_eq!(format!("{}", expr), "(group (7))");
    }

    #[test]
    fn bodmas() {
        // 6 / 3 - 1
        let expr = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(6.0),
                }),
                operator: Token::new(TokenType::Slash, "/".to_string(), None, 0),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(3.0),
                }),
            }),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(1.0),
            }),
        };
        assert_eq!(
            format!("{}", expr),
            "(((6) Slash / None (3)) Minus - None (1))"
        );
    }
}
