use std::collections::HashMap;
use crate::ast::*;

pub fn eval_program(tree: Program) -> isize {
    let mut vars = HashMap::new();
    for assign in tree.body {
        let (name, val) = eval_assign(assign, &vars);
        vars.insert(name, val);
    }
    match vars.get(&"result".to_string()) {
        Some(a) => *a,
        None => match vars.into_iter().last() {
            Some(a) => a.1,
            None => 0,
        }
    }
}

pub fn eval_assign(a: Assign, vars: &HashMap<String, isize>) -> (String, isize) {
    let name = a.0;
    let value = eval_expr(a.1, vars);
    (name, value)
}

pub fn eval_expr(e: Expression, vars: &HashMap<String, isize>) -> isize {
    match e {
        Expression::Var(v) => {
            match vars.get(&v) {
                Some(i) => *i,
                None => panic!("Erro: Variável {v} não existe!"),
            }
        }
        Expression::Num(i) => i,
        Expression::Parenthed(f) => eval_expr(*f, vars),
        Expression::Operation(op, exprs) => eval_operation(op, exprs, vars),
    }
}

pub fn eval_operation(op: String, exprs: Vec<Expression>, vars: &HashMap<String, isize>) -> isize {
    match op.as_str() {
        "+" => match exprs.len() {
            1 => eval_expr(exprs[0].clone(), vars),
            2 => eval_expr(exprs[0].clone(), vars) + eval_expr(exprs[1].clone(), vars),
            _ => panic!("invalid size of args: !1 && !2: {exprs:?}")
        }
        "-" => match exprs.len() {
            1 => -(eval_expr(exprs[0].clone(), vars)),
            2 => eval_expr(exprs[0].clone(), vars) - eval_expr(exprs[1].clone(), vars),
            _ => panic!("invalid size of args: !1 && !2: {exprs:?}")
        }
        "*" => eval_expr(exprs[0].clone(), vars) * eval_expr(exprs[1].clone(), vars),
        "/" => eval_expr(exprs[0].clone(), vars) / eval_expr(exprs[1].clone(), vars),
        _ => panic!("unexpected operator in eval: {op}"),
    }
}
