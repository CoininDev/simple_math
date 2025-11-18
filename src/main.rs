use crate::eval::eval_program;
use crate::repl::REPL;
use crate::{
    ast::Parser,
    lexer::{Lexer, Token},
};
use std::{env, fs};
mod ast;
mod eval;
mod lexer;
mod repl;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    match args.first() {
        Some(a) if a == "-i" => {
            println!("<== Welcome to Simple Interactive Mode ==>");
            let mut repl = REPL::new();
            loop {
                if repl.step() {
                    break;
                }
            }
        }

        Some(a) if a == "-f" => {
            let f_flag_pos = args.iter().position(|x| x == "-f").unwrap();
            let path = args
                .get(f_flag_pos + 1)
                .expect("Please, provide a path after `-f`");
            let content =
                fs::read_to_string(path).expect(format!("Error reading file {path}").as_str());

            let tk = tokenize(content.as_str());
            // dbg!(&tk);
            let mut parser = Parser::new(tk);
            let program = parser.parse_program();
            // dbg!(&program);
            let result = eval_program(program);

            println!("result = {result}");
        }

        None | Some(_) => usage(),
    }
}

fn usage() {
    let message = "Usage: simple_math [OPTIONS]

Options:
  -i             Enter interactive mode
  -f <filename>  Evaluate a file and print the result
    ";
    println!("{message}");
}

fn tokenize(s: &str) -> Vec<Token> {
    let lex = Lexer::new(s);
    let mut vlex = vec![];
    for t in lex {
        vlex.push(t);
    }
    vlex
}
