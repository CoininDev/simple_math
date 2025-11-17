use std::{error::Error, fmt::Display};

use crate::lexer::{Token, TokenType, binding_power, is_valid_unary, unary_binding_power};

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Assign>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for a in self.clone().body {
            write!(f, "{a}\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Var(String),
    Num(isize),
    Parenthed(Box<Expression>),
    Operation(String, Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Var(s) => write!(f, "%{s}"),
            Expression::Num(i) => write!(f, "{i}"),
            Expression::Parenthed(a) => write!(f, "({a})"),
            Expression::Operation(op, e) => {
                write!(f, "({op}\n")?;
                for expr in e {
                    write!(f, "\t{expr}\n")?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assign(pub String, pub Expression);

impl Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.0, self.1)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug)]
pub enum ParsingError {
    Unexpected(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::Unexpected(a) => f.write_str(format!("Unexpected: {}", a).as_str()),
        }
    }
}

impl Error for ParsingError {}

impl Parser {
    pub fn peek(&self, p: usize) -> Option<&Token> {
        self.tokens.get(self.pos + p)
    }

    pub fn peek_type(&self, p: usize) -> Option<&TokenType> {
        let t = self.tokens.get(self.pos + p);
        if t.is_none() {
            return None;
        }
        Some(&t.unwrap().token_type)
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.pos > self.tokens.len() {
            return None;
        }

        let t = self.tokens[self.pos].to_owned();
        self.pos += 1;
        Some(t)
    }

    pub fn expect(&mut self, expected: TokenType) -> Result<(), ParsingError> {
        if let Some(pook) = self.peek(0) {
            if pook.token_type == expected {
                self.next();
                return Ok(());
            } else {
                return Err(ParsingError::Unexpected(format!("{:?}", self.peek(0))));
            }
        }
        Err(ParsingError::Unexpected(format!("{:?}", self.peek(0))))
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut buf = vec![];
        while self.peek(0).is_some() {
            if self.peek_type(0) == Some(&TokenType::EndExpr) {
                self.next();
                continue;
            }
            let a = self.parse_assign();
            match self.expect(TokenType::EndExpr) {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {e}"),
            };

            buf.push(a);
        }

        Program { body: buf }
    }

    pub fn parse_assign(&mut self) -> Assign {
        let id = match self.peek_type(0) {
            Some(TokenType::Ident(a)) => a.clone(),
            Some(TokenType::EndExpr) => {
                self.next();
                return self.parse_assign();
            }
            _ => panic!("parse_assign: Expected ident found {:?}", self.peek(0)),
        };
        self.next();

        self.expect(TokenType::Assign)
            .expect("parse_assign expected '='.");

        let expr = self.parse_expr_pratt(0.);

        Assign(id, expr)
    }

    pub fn parse_expr_pratt(&mut self, min_bp: f32) -> Expression {
        let mut lhs = match self.next() {
            Some(Token {
                token_type: TokenType::Number(n),
                ..
            }) => Expression::Num(n),
            Some(Token {
                token_type: TokenType::Ident(i),
                ..
            }) => Expression::Var(i),

            Some(Token {
                token_type: TokenType::LParen,
                ..
            }) => {
                let expr = self.parse_expr_pratt(0.);
                match self.next() {
                    Some(Token {
                        token_type: TokenType::RParen,
                        ..
                    }) => Expression::Parenthed(Box::new(expr)),
                    other => panic!("Expected ')' but found : {other:?}"),
                }
            }

            Some(Token {
                token_type: TokenType::Op(op),
                ..
            }) if is_valid_unary(op.as_str()) => {
                let (_, bp_r) = unary_binding_power(&op);
                let rhs = self.parse_expr_pratt(bp_r);
                Expression::Operation(op.clone(), vec![rhs])
            }
            e => panic!("Unexpected: {e:?}"),
        };

        loop {
            let op = match self.peek_type(0) {
                None | Some(TokenType::EndExpr) | Some(TokenType::RParen) => break,
                Some(TokenType::Op(op)) => op.clone(),
                t => panic!("bad token: {t:?}"),
            };
            let (bp_l, bp_r) = binding_power(op.as_str());
            if bp_l < min_bp {
                break;
            }
            self.next();
            let rhs = self.parse_expr_pratt(bp_r);
            lhs = Expression::Operation(op.to_owned(), vec![lhs, rhs]);
        }

        lhs
    }
}
