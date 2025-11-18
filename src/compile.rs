use std::collections::HashMap;

use qbe::{Block, Function, Instr, Linkage, Module, Type, Value};

use crate::ast::{Assign, Expression, Program};

fn op_name(op: &str) -> &str {
    match op {
        "+" => "add",
        "-" => "sub",
        "/" => "div",
        "*" => "mul",
        _ => "unknown",
    }
}

fn gen_qbe_var_name(op: &str, sid: usize) -> String {
    format!("{op}_{sid}")
}

pub struct QBEParser<'lt> {
    op_counts: HashMap<String, usize>,
    module: Module<'lt>,
}

impl<'lt> QBEParser<'lt> {
    fn get_count(&mut self, op: &str) -> usize {
        match self.op_counts.get(op) {
            Some(i) => *i,
            None => {
                self.op_counts.insert(op.to_string(), 0);
                0
            }
        }
    }

    fn increment_count(&mut self, op: &str) {
        match self.op_counts.get(op) {
            Some(_) => *self.op_counts.get_mut(op).unwrap() += 1,
            None => {
                let _ = self.op_counts.insert(op.to_string(), 1);
            }
        }
    }

    pub fn write_program(&mut self, tree: Program) {
        self.module = Module::new();
        for assign in tree.body {
            self.write_assign(assign);
        }
    }

    pub fn write_assign(&mut self, a: Assign) {
        let name = a.0;
        let mut func = Function::new(Linkage::private(), name.clone(), vec![], Some(Type::Double));
        let mut block = func.add_block("start");
        self.write_expr(a.1, block);
        if name == "result" {
            /*main function*/
            return;
        }
    }

    pub fn write_expr(&mut self, e: Expression, block: &mut Block) {
        match e {
            Expression::Var(v) => {
                let count = self.get_count(v.as_str());
                let name = gen_qbe_var_name(v.as_str(), count).to_string();

                block.assign_instr(
                    Value::Temporary(name.clone()),
                    Type::Double,
                    Instr::Call(name, vec![], None),
                );
            }
            Expression::Num(i) => {}
            Expression::Parenthed(f) => self.write_expr(*f, block),
            Expression::Operation(op, exprs) => self.write_operation(op, exprs, block),
        };
    }

    pub fn write_operation(&mut self, op: String, exprs: Vec<Expression>, block: &mut Block) {
        match exprs.len() {
            2 => {}
            1 => {}

            _ => panic!("compilador: AAAAAAAAAAAAAAAAAAAAAAAAAa"),
        }
    }
}
