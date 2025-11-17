use rustyline::{DefaultEditor, error::ReadlineError};
use std::collections::HashMap;

use crate::{lexer::{Lexer, Token}, ast::Parser, eval::*};

pub struct REPL {
    vars: HashMap<String, isize>,
    rl: DefaultEditor,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            rl: DefaultEditor::new().unwrap()
        }
    }
    
    pub fn step(&mut self) -> bool {
        match self.rl.readline("> ") {
            Ok(line) => {
                let tk = self.tokenize(line.as_str());
                let mut parser = Parser::new(tk);
                if line.contains("=") {
                    let assign = parser.parse_assign();
                    let (n, v) = eval_assign(assign, &self.vars);
                    println!("< {n} = {v}");
                    self.vars.insert(n, v);
                } else {
                    let v = self.vars.get(&line);
                    match v {
                        Some(a) => println!("= {a}"),
                        None => eprintln!("This variable does not exist"),
                    }
                }

                self.rl.add_history_entry(&line).unwrap();

                println!("<-------------------------->");
                false
            }
            Err(ReadlineError::Interrupted) => true,
            Err(ReadlineError::Eof) => true,
            Err(e) => {
                eprintln!("Erro: {e}");
                true
            }
        }
    }

    pub fn tokenize(&self, s: &str) -> Vec<Token> {
        let lex = Lexer::new(s);
        let mut vlex = vec![];
        for t in lex {
            vlex.push(t);
        }
        vlex
    }
}
