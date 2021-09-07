use crate::callable::Function;
use crate::environment::EnvStack;
use crate::parser::{Expr, Stmt};
use crate::token::{Token, TokenType};
use crate::value::{Er, Value};

use std::collections::HashMap;

fn cast_to_num(v: &Value) -> Result<f64, Er> {
    if let Value::Number(n) = v {
        Ok(*n)
    } else {
        Err(Er::Code(12))
    }
}

fn cast_to_string(v: &Value) -> Result<&String, Er> {
    if let Value::Text(s) = v {
        Ok(s)
    } else {
        Err(Er::Code(21))
    }
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Text(_) => true,
        Value::Number(_) => true,
        Value::Boolean(b) => *b,
        Value::Nil => false,
        Value::Callable(_) => true,
    }
}

fn is_equal(lhs: &Value, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (Value::Nil, Value::Nil) => true,
        _ => lhs == rhs,
    }
}

#[derive(Debug, Default)]
pub struct Interpreter {
    pub envs: EnvStack<Value>,
    pub locals: HashMap<Expr, usize>,
}

impl Interpreter {
    fn eval_binary(&mut self, token: &Token, lhs: &Expr, rhs: &Expr) -> Result<Value, Er> {
        let lhs_val = self.eval_expr(lhs)?;
        let rhs_val = self.eval_expr(rhs)?;

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
            TokenType::Greater => Value::Boolean(cast_to_num(&lhs_val)? > cast_to_num(&rhs_val)?),
            TokenType::GreaterEqual => {
                Value::Boolean(cast_to_num(&lhs_val)? >= cast_to_num(&rhs_val)?)
            }
            TokenType::Less => Value::Boolean(cast_to_num(&lhs_val)? < cast_to_num(&rhs_val)?),
            TokenType::LessEqual => {
                Value::Boolean(cast_to_num(&lhs_val)? <= cast_to_num(&rhs_val)?)
            }
            TokenType::EqualEqual => Value::Boolean(is_equal(&lhs_val, &rhs_val)),
            TokenType::BangEqual => Value::Boolean(!is_equal(&lhs_val, &rhs_val)),
            _ => {
                return Err(Er::Code(14));
            }
        })
    }

    fn eval_logical(&mut self, token: &Token, lhs: &Expr, rhs: &Expr) -> Result<Value, Er> {
        let lhs_val = self.eval_expr(lhs)?;

        Ok(match token.token_type {
            TokenType::Or => {
                if is_truthy(&lhs_val) {
                    lhs_val
                } else {
                    self.eval_expr(rhs)?
                }
            }
            TokenType::And => {
                if !is_truthy(&lhs_val) {
                    lhs_val
                } else {
                    self.eval_expr(rhs)?
                }
            }
            _ => {
                return Err(Er::Code(55));
            }
        })
    }

    fn eval_unary(&mut self, token: &Token, rhs: &Expr) -> Result<Value, Er> {
        let rhs_val = cast_to_num(&self.eval_expr(rhs)?)?;

        match token.token_type {
            TokenType::Minus => Ok(Value::Number(-rhs_val)),
            _ => Err(Er::Code(14)),
        }
    }

    fn eval_assign(&mut self, name: &str, rhs: &Expr) -> Result<Value, Er> {
        let rhs_val = self.eval_expr(rhs)?;

        self.envs.assign(name, rhs_val.clone())?;

        Ok(rhs_val)
    }

    fn eval_leaf(&self, token: &Token) -> Result<Value, Er> {
        Ok(match &token.token_type {
            TokenType::Text(s) => Value::Text(s.clone()),
            TokenType::Number(n) => Value::Number(n.parse::<f64>().unwrap()), // safe unwrap as scanner checks that the string can be converted to a number
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            TokenType::Nil => Value::Nil,
            TokenType::Identifier(s) => self.envs.get(s.as_str())?.clone(),
            _ => {
                return Err(Er::Code(13));
            }
        })
    }

    fn eval_call(&mut self, callee: &Expr, args: &Vec<Expr>) -> Result<Value, Er> {
        let call = match self.eval_expr(callee)? {
            Value::Callable(c) => c,
            _ => {
                // return an Error on non-callable types returned from callee
                return Err(Er::Code(1060));
            }
        };

        let mut evaled_args = vec![];
        for arg in args {
            evaled_args.push(self.eval_expr(arg)?);
        }

        if call.artiy() != evaled_args.len() {
            return Err(Er::Code(1061));
        }

        call.call(self, evaled_args)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Er> {
        match expr {
            Expr::Leaf(t) => self.eval_leaf(t),
            Expr::Assign(s, rhs) => self.eval_assign(s, rhs),
            Expr::Unary(t, rhs) => self.eval_unary(t, rhs),
            Expr::Binary(t, lhs, rhs) => self.eval_binary(t, lhs, rhs),
            Expr::Logical(t, lhs, rhs) => self.eval_logical(t, lhs, rhs),
            Expr::Grouping(expr) => self.eval_expr(expr),
            Expr::Call(_, callee, args) => self.eval_call(callee, args),
        }
    }

    fn eval_print(&mut self, expr: &Expr) -> Result<(), Er> {
        let val = self.eval_expr(expr)?;
        println!("{}", val.to_string());
        Ok(())
    }

    fn eval_fun_decl(&mut self, fun: Function) -> Result<(), Er> {
        self.envs.define(
            fun.declaration.name.clone().as_str(),
            Value::Callable(std::rc::Rc::new(fun)),
        );
        Ok(())
    }

    fn eval_decl(&mut self, name: &String, expr: &Expr) -> Result<(), Er> {
        let rhs = self.eval_expr(expr)?;
        self.envs.define(&name.as_str(), rhs);
        Ok(())
    }

    pub fn eval_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), Er> {
        self.envs.push_default();
        // take eval_res here to ensure we always call pop even on failure
        // could use defer crate or similar for pop instead!
        for stmt in stmts {
            let res = self.evaluate(stmt);
            if res.is_err() {
                self.envs.pop()?;
                return res;
            }
        }
        self.envs.pop()?;
        Ok(())
    }

    fn eval_if(&mut self, cond: &Expr, lhs: &Stmt, rhs: &Option<Stmt>) -> Result<(), Er> {
        if is_truthy(&self.eval_expr(cond)?) {
            self.evaluate(lhs)?;
        } else if let Some(rhs_expr) = rhs {
            self.evaluate(rhs_expr)?;
        }

        Ok(())
    }

    fn eval_while(&mut self, cond: &Expr, body: &Stmt) -> Result<(), Er> {
        while is_truthy(&self.eval_expr(cond)?) {
            self.evaluate(body)?;
        }

        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        // this might just be bugged still, as Tokens in expression don't hold a reference to the lexeme, only kind and line
        // question is whether there is a case where kind and line are equal, but resolved var is different? (e.g. whole program in one line?)
        self.locals.insert(expr.clone(), depth);
    }

    pub fn evaluate(&mut self, stmt: &Stmt) -> Result<(), Er> {
        match stmt {
            Stmt::Expression(expr) => {
                self.eval_expr(expr)?;
            }
            Stmt::Function(stmt_function) => {
                self.eval_fun_decl(Function {
                    declaration: stmt_function.clone(),
                })?;
            }
            Stmt::Return(expr) => {
                return Err(Er::Return(self.eval_expr(expr)?));
            }
            Stmt::Print(expr) => {
                self.eval_print(expr)?;
            }
            Stmt::Var(token, expr) => {
                self.eval_decl(token, expr)?;
            }
            Stmt::Block(stmts) => {
                self.eval_block(stmts)?;
            }
            Stmt::If(cond, lhs, rhs) => {
                self.eval_if(cond, &*lhs, &*rhs)?;
            }
            Stmt::While(cond, body) => {
                self.eval_while(cond, &*body)?;
            }
        }

        Ok(())
    }
}
