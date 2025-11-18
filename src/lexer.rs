pub struct Lexer {
    text: String,
    pos: usize,
    current_line: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        let mut text = String::from(text);
        text = text.replace(" ", "");
        text = text.replace("\t", "");
        Self {
            text,
            pos: 0,
            current_line: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: usize,
    pub token_type: TokenType,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    LParen,
    RParen,
    Op(String),
    Assign,
    Ident(String),
    EndExpr,
}

pub fn binding_power(op: &str) -> (f32, f32) {
    match op {
        "+" | "-" => (1., 1.1),
        "*" | "/" => (2., 2.1),
        _ => (3., 3.1),
    }
}

pub fn is_valid_unary(op: &str) -> bool {
    let valid = vec!["+", "-", "!"];
    valid.contains(&op)
}

pub fn unary_binding_power(op: &str) -> (f32, f32) {
    match op {
        "-" | "+" | "!" => (3.0, 3.1),
        _ => panic!("unknown unary op: {op}"),
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.text.len() {
            return None;
        }
        let slice = &self.text[self.pos..];
        let ch = slice.chars().next().unwrap();
        let ch_len = ch.len_utf8();

        let advance = |this: &mut Lexer, nbytes: usize| {
            this.pos += nbytes;
        };

        advance(self, ch_len);
        match ch {
            '=' => Some(Token {
                line: self.current_line,
                token_type: TokenType::Assign,
            }),
            '+' | '-' | '*' | '/' => Some(Token {
                line: self.current_line,
                token_type: TokenType::Op(ch.to_string()),
            }),
            '(' => Some(Token {
                line: self.current_line,
                token_type: TokenType::LParen,
            }),
            ')' => Some(Token {
                line: self.current_line,
                token_type: TokenType::RParen,
            }),
            '\n' => {
                self.current_line += 1;
                Some(Token {
                    line: self.current_line,
                    token_type: TokenType::EndExpr,
                })
            }

            d if d.is_ascii_digit() || d == '.' => {
                let mut seen_dot = d == '.';
                let mut buf = String::new();
                buf.push(d);

                // Avança após consumir o primeiro caractere
                //advance(self, d.len_utf8());

                while self.pos < self.text.len() {
                    let next_slice = &self.text[self.pos..];
                    let next_ch = next_slice.chars().next().unwrap();

                    if next_ch.is_ascii_digit() {
                        buf.push(next_ch);
                        advance(self, next_ch.len_utf8());
                    } else if next_ch == '.' {
                        if seen_dot {
                            eprintln!(
                                "Mais de um ponto detectado no número \"{}\"; ignorando ponto extra",
                                buf
                            );
                            advance(self, next_ch.len_utf8());
                        } else {
                            seen_dot = true;
                            buf.push('.');
                            advance(self, next_ch.len_utf8());
                        }
                    } else {
                        break;
                    }
                }
                let number = buf.parse::<f64>().unwrap_or_else(|e| {
                    eprintln!("Erro ao converter \"{buf}\" para float: {e}");
                    0.0
                });

                return Some(Token {
                    line: self.current_line,
                    token_type: TokenType::Number(number),
                });
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut buf = String::from(c);
                while self.pos < self.text.len() {
                    let next_slice = &self.text[self.pos..];
                    let next_ch = next_slice.chars().next().unwrap();
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        buf.push(next_ch);
                        advance(self, next_ch.len_utf8());
                    } else {
                        break;
                    }
                }
                Some(Token {
                    line: self.current_line,
                    token_type: TokenType::Ident(buf),
                })
            }
            _ => {
                eprintln!(
                    "unhandled character at: {}, position: {}",
                    self.current_line, self.pos,
                );
                self.next()
            }
        }
    }
}
