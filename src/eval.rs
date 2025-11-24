use crate::{ast::*, error::EvalError};
use std::{collections::HashMap, fmt};

pub type EvalResult<T> = Result<T, EvalError>;

pub fn eval_program(tree: Program) -> EvalResult<f64> {
    let mut vars = HashMap::new();
    for assign in tree.body {
        let (name, val) = eval_assign(assign, &vars)?;
        vars.insert(name, val);
    }
    match vars.get(&"result".to_string()) {
        Some(a) => Ok(*a),
        None => match vars.into_iter().last() {
            Some(a) => Ok(a.1),
            None => Ok(0.),
        },
    }
}

pub fn eval_assign(a: Assign, vars: &HashMap<String, f64>) -> EvalResult<(String, f64)> {
    let name = a.0;
    let value = eval_expr(a.1, vars)?;
    Ok((name, value))
}

pub fn eval_expr(e: Expression, vars: &HashMap<String, f64>) -> EvalResult<f64> {
    match e {
        Expression::Var(v) => match vars.get(&v) {
            Some(i) => Ok(*i),
            None => Err(EvalError::VariableDoesNotExists(format!("{v}"))),
        },
        Expression::Num(i) => Ok(i),
        Expression::Parenthed(f) => eval_expr(*f, vars),
        Expression::Operation(op, exprs) => eval_operation(op, exprs, vars),
    }
}

pub fn eval_operation(op: String, exprs: Vec<Expression>, vars: &HashMap<String, f64>) -> EvalResult<f64> {
    match op.as_str() {
        "+" => match exprs.len() {
            1 => eval_expr(exprs[0].clone(), vars),
            2 => Ok(eval_expr(exprs[0].clone(), vars)? + eval_expr(exprs[1].clone(), vars)?),
            _ => Err(EvalError::InvalidSizeOfArgsFor("+".to_string())),
        },
        "-" => match exprs.len() {
            1 => Ok(-(eval_expr(exprs[0].clone(), vars)?)),
            2 => Ok(eval_expr(exprs[0].clone(), vars)? - eval_expr(exprs[1].clone(), vars)?),
            _ => Err(EvalError::InvalidSizeOfArgsFor("-".to_string())),
        },
        "*" => Ok(eval_expr(exprs[0].clone(), vars)? * eval_expr(exprs[1].clone(), vars)?),
        "/" => match eval_expr(exprs[1].clone(), vars) { 
            Ok(0.0) => Err(EvalError::ZeroDivisor),
            _ => Ok(eval_expr(exprs[0].clone(), vars)? / eval_expr(exprs[1].clone(), vars)?) },
        _ => Err(EvalError::UnexpectedOperator(format!("{op}"))),
    }
}
