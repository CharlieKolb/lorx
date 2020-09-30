use crate::parser::Expr;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

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

fn eval_binary(token: Token, lhs: Box<Expr>, rhs: Box<Expr>) -> Result<Value, usize> {
    let lhs_val = evaluate(*lhs)?;
    let rhs_val = evaluate(*rhs)?;

    Ok(match token.token_type {
        TokenType::Minus => Value::Number(cast_to_num(&lhs_val)? - cast_to_num(&rhs_val)?),
        TokenType::Slash => Value::Number(cast_to_num(&lhs_val)? / cast_to_num(&rhs_val)?),
        TokenType::Star => Value::Number(cast_to_num(&lhs_val)? * cast_to_num(&rhs_val)?),
        TokenType::Plus => {
            let lhs_num = cast_to_num(&lhs_val);
            let rhs_num = cast_to_num(&rhs_val);

            if lhs_num.is_ok() && rhs_num.is_ok() {
                Value::Number(lhs_num.unwrap() + rhs_num.unwrap())
            }
            else {
                Value::Text(cast_to_string(&lhs_val)?.clone() + cast_to_string(&rhs_val)?)
            }
        },
        TokenType::Greater => Value::Boolean(cast_to_num(&lhs_val) > cast_to_num(&rhs_val)),
        TokenType::GreaterEqual => Value::Boolean(cast_to_num(&lhs_val) >= cast_to_num(&rhs_val)),
        TokenType::Less => Value::Boolean(cast_to_num(&lhs_val) < cast_to_num(&rhs_val)),
        TokenType::LessEqual => Value::Boolean(cast_to_num(&lhs_val) <= cast_to_num(&rhs_val)),
        TokenType::EqualEqual => Value::Boolean(is_equal(&lhs_val, &rhs_val)),
        TokenType::BangEqual => Value::Boolean(!is_equal(&lhs_val, &rhs_val)),
        _ => { return Err(14); },
    })
}

fn eval_unary(token: Token, rhs: Box<Expr>) -> Result<Value, usize> {
    let rhs_val = cast_to_num(&evaluate(*rhs)?)?;

    match token.token_type {
        TokenType::Minus => Ok(Value::Number(-rhs_val)),
        _ => Err(14),
    }
}

fn eval_leaf(token: Token) -> Result<Value, usize> {
    Ok(match token.token_type {
        TokenType::Text(s) => Value::Text(s),
        TokenType::Number(n) => Value::Number(n),
        _ => {
            return Err(13);
        }
    })
}

pub fn evaluate(expr: Expr) -> Result<Value, usize> {
    match expr {
        Expr::Leaf(t) => eval_leaf(t),
        Expr::Unary(t, rhs) => eval_unary(t, rhs),
        Expr::Binary(t, lhs, rhs) => eval_binary(t, lhs, rhs),
        Expr::Grouping(expr) => evaluate(*expr),
        _ => Err(17),
    }
}
