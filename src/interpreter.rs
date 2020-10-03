use crate::parser::{Expr, Stmt};
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::environment::Environment;

fn cast_to_num(v: &Value) -> Result<f64, usize> {
    if let Value::Number(n) = v {
        Ok(*n)
    } else {
        Err(12)
    }
}

fn cast_to_string(v: &Value) -> Result<&String, usize> {
    if let Value::Text(s) = v {
        Ok(s)
    } else {
        Err(21)
    }
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Text(_) => true,
        Value::Number(_) => true,
        Value::Boolean(b) => *b,
        Value::Nil => false,
    }
}

fn is_equal(lhs: &Value, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (Value::Nil, Value::Nil) => true,
        _ => lhs == rhs,
    }
}

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    fn eval_binary(&self, token: Token, lhs: Box<Expr>, rhs: Box<Expr>) -> Result<Value, usize> {
        let lhs_val = self.eval_expr(*lhs)?;
        let rhs_val = self.eval_expr(*rhs)?;

        Ok(match token.token_type {
            TokenType::Minus => Value::Number(cast_to_num(&lhs_val)? - cast_to_num(&rhs_val)?),
            TokenType::Slash => Value::Number(cast_to_num(&lhs_val)? / cast_to_num(&rhs_val)?),
            TokenType::Star => Value::Number(cast_to_num(&lhs_val)? * cast_to_num(&rhs_val)?),
            TokenType::Plus => {
                let lhs_num = cast_to_num(&lhs_val);
                let rhs_num = cast_to_num(&rhs_val);

                if lhs_num.is_ok() && rhs_num.is_ok() {
                    Value::Number(lhs_num.unwrap() + rhs_num.unwrap())
                } else {
                    Value::Text(cast_to_string(&lhs_val)?.clone() + cast_to_string(&rhs_val)?)
                }
            }
            TokenType::Greater => Value::Boolean(cast_to_num(&lhs_val) > cast_to_num(&rhs_val)),
            TokenType::GreaterEqual => Value::Boolean(cast_to_num(&lhs_val) >= cast_to_num(&rhs_val)),
            TokenType::Less => Value::Boolean(cast_to_num(&lhs_val) < cast_to_num(&rhs_val)),
            TokenType::LessEqual => Value::Boolean(cast_to_num(&lhs_val) <= cast_to_num(&rhs_val)),
            TokenType::EqualEqual => Value::Boolean(is_equal(&lhs_val, &rhs_val)),
            TokenType::BangEqual => Value::Boolean(!is_equal(&lhs_val, &rhs_val)),
            _ => {
                return Err(14);
            }
        })
    }

    fn eval_unary(&self, token: Token, rhs: Box<Expr>) -> Result<Value, usize> {
        let rhs_val = cast_to_num(&self.eval_expr(*rhs)?)?;

        match token.token_type {
            TokenType::Minus => Ok(Value::Number(-rhs_val)),
            _ => Err(14),
        }
    }

    fn eval_leaf(& self, token: Token) -> Result<Value, usize> {
        Ok(match token.token_type {
            TokenType::Text(s) => Value::Text(s),
            TokenType::Number(n) => Value::Number(n),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            TokenType::Nil => Value::Nil,
            TokenType::Identifier(s) => self.environment.get(s.as_str())?.clone(),
            _ => {
                return Err(13);
            }
        })
    }

    fn eval_expr(& self, expr: Expr) -> Result<Value, usize> {
        match expr {
            Expr::Leaf(t) =>self. eval_leaf(t),
            Expr::Unary(t, rhs) => self.eval_unary(t, rhs),
            Expr::Binary(t, lhs, rhs) => self.eval_binary(t, lhs, rhs),
            Expr::Grouping(expr) => self.eval_expr(*expr),
            _ => Err(17),
        }
    }

    fn eval_print(&self, expr: Expr) -> Result<(), usize> {
        let val = self.eval_expr(expr)?;
        println!("{}", val.to_string());
        Ok(())
    }

    fn eval_decl(&mut self, token: Token, expr: Expr) -> Result<(), usize> {
        if let Token { token_type: TokenType::Identifier(s), .. } = token {
            self.environment.define(&s.as_str(), self.eval_expr(expr)?);
            Ok(())
        }
        else {
            Err(35)
        }
    }

    pub fn evaluate(&mut self, stmts: Vec<Stmt>) -> Result<(), usize> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression(expr) => {
                    self.eval_expr(expr)?;
                }
                Stmt::Print(expr) => {
                    self.eval_print(expr)?;
                }
                Stmt::Var(token, expr) => {
                    self.eval_decl(token, expr)?;
                }
            }
        }

        Ok(())
    }
}
