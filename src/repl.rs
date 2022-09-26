use crate::parser::{Expr, Op, Type};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq,)]
pub enum Value {
    Number(f64),
    Vector(Vec<Box<Value>>),
    Matrix(Vec<Box<Value>>),
    Void,
}

pub struct Repl {
    definitions: HashMap<Declaration, Expr>,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            definitions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::BinaryExpr {..} => self.binary_expr(expr),
            _ => Value::Void 
        }
    }

    pub fn binary_expr(&mut self, expr: Expr) -> Value {
        if let Expr::BinaryExpr { lhs, op, rhs } = expr {
            match op {
                Op::Define => {
                    if let Expr::Declaration {name, ty} = *lhs {
                        self.definitions.insert(Declaration {
                            name,
                            ty,
                        }, *rhs);

                        return Value::Void;
                    }         
                }
                _ => ()
            }

            let lhs = self.interpret(*lhs);
            let rhs = self.interpret(*rhs);
            match op {
                Op::Plus => {
                    if let Value::Number(lhs) = lhs {
                        if let Value::Number(rhs) = rhs {
                            Value::Number(lhs + rhs)
                        } else {
                            Value::Void
                        }
                    } else {
                        Value::Void
                    }
                } 
                Op::Minus => {
                    if let Value::Number(lhs) = lhs {
                        if let Value::Number(rhs) = rhs {
                            Value::Number(lhs - rhs)
                        } else {
                            Value::Void
                        }
                    } else {
                        Value::Void
                    }
                }
                Op::Star => {
                    if let Value::Number(lhs) = lhs {
                        if let Value::Number(rhs) = rhs {
                            Value::Number(lhs * rhs)
                        } else {
                            Value::Void
                        }
                    } else {
                        Value::Void
                    }
                }
                Op::Slash => {
                    if let Value::Number(lhs) = lhs {
                        if let Value::Number(rhs) = rhs {
                            Value::Number(lhs / rhs)
                        } else {
                            Value::Void
                        }
                    } else {
                        Value::Void
                    }
                }
                
                _ => Value::Void
            }
        } else {
            Value::Void
        }
    }
}
