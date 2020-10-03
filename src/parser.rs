use crate::token::{Token, TokenType};

use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Expr {
    Leaf(Token),
    // Variable(Token), // probably won't need this?
    Assign(String, Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
}

fn match_next(
    iter: &mut Peekable<impl Iterator<Item = Token>>,
    options: &[TokenType],
) -> Option<Token> {
    for token_type in options {
        if iter.peek().map(|t| &t.token_type) == Some(&token_type) {
            return iter.next();
        };
    }
    None
}

fn parse_primary(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    if let Some(op) = match_next(
        &mut iter,
        &[TokenType::False, TokenType::True, TokenType::Nil],
    ) {
        return Ok(Expr::Leaf(op));
    }

    if let Some(_) = match_next(&mut iter, &[TokenType::LeftParen]) {
        let expr = parse_expression(&mut iter)?;

        if match_next(&mut iter, &[TokenType::RightParen]).is_none() {
            return Err(1); // Missing closing brace
        }

        return Ok(Expr::Grouping(Box::new(expr)));
    }

    // Only options left are literals/variables
    if let Some(t) = iter.next() {
        Ok(match t.token_type {
            TokenType::Number(_)
            | TokenType::Text(_)
            | TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::Identifier(_) => Expr::Leaf(t),
            _ => return Err(33),
        })
    } else {
        Err(0) // Ran out of elements
    }
}

fn parse_unary(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    if let Some(op) = match_next(&mut iter, &[TokenType::Bang, TokenType::Minus]) {
        let rhs = parse_unary(&mut iter)?;
        return Ok(Expr::Unary(op, Box::new(rhs)));
    }

    parse_primary(&mut iter)
}

fn parse_multiplication(
    mut iter: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expr, usize> {
    let mut lhs = parse_unary(&mut iter)?;
    while let Some(op) = match_next(&mut iter, &[TokenType::Slash, TokenType::Star]) {
        let rhs = parse_unary(&mut iter)?;
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_addition(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    let mut lhs = parse_multiplication(&mut iter)?;
    while let Some(op) = match_next(&mut iter, &[TokenType::Plus, TokenType::Minus]) {
        let rhs = parse_multiplication(&mut iter)?;
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_comparison(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    let mut lhs = parse_addition(&mut iter)?;
    while let Some(op) = match_next(
        &mut iter,
        &[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ],
    ) {
        let rhs = parse_addition(&mut iter)?;
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_equality(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    let mut lhs = parse_comparison(&mut iter)?;
    while let Some(op) = match_next(&mut iter, &[TokenType::EqualEqual, TokenType::BangEqual]) {
        let rhs = parse_comparison(&mut iter)?;
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_print(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Stmt, usize> {
    if match_next(&mut iter, &[TokenType::Print]).is_some() {
        let expr = parse_expression(&mut iter)?;
        if match_next(&mut iter, &[TokenType::Semicolon]).is_none() {
            Err(22)
        } else {
            Ok(Stmt::Print(expr))
        }
    } else {
        Err(21)
    }
}

fn parse_assignment(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    let expr = parse_equality(&mut iter)?;

    if match_next(&mut iter, &[TokenType::Equal]).is_some() {
        let rhs = parse_assignment(&mut iter)?;

        match expr {
            Expr::Leaf(Token { token_type: TokenType::Identifier(s), .. }) =>
                Ok(Expr::Assign(s, Box::new(rhs))),
            _ => Err(42),

        }
    }
    else {
        Ok(expr)
    }


    // match iter.peek().map(|t| &t.token_type) {
    //     Some(&TokenType::Identifier(_)) => {
    //         let lhs = iter.next().unwrap(); // peeked in the match
    //         if match_next(&mut iter, &[TokenType::Equal]).is_some() {
    //             let rhs = parse_assignment(&mut iter)?;
    //             Ok(Expr::Assign(lhs, Box::new(rhs)))
    //         }
    //         else {
    //             Err(41)
    //         }
    //     }
    //     _ => parse_equality(&mut iter)
    // }
}

fn parse_expression(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, usize> {
    parse_assignment(&mut iter)
}

fn parse_exprstmt(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Stmt, usize> {
    let expr = parse_expression(&mut iter)?;
    if match_next(&mut iter, &[TokenType::Semicolon]).is_none() {
        Err(22)
    } else {
        Ok(Stmt::Expression(expr))
    }
}

fn parse_block(mut iter:  &mut Peekable<impl Iterator<Item = Token>>) -> Result<Stmt, usize> {
    if match_next(&mut iter, &[TokenType::LeftBrace]).is_none() {
       Err(22)
    }
    else {
        let mut stmts = vec![];
        while iter.peek().is_some() && iter.peek().map(|t| &t.token_type) != Some(&TokenType::RightBrace) {
            stmts.push(parse_decl(&mut iter)?);
        }
        // drop right brace
        iter.next();
        Ok(Stmt::Block(stmts))
    }
}

fn parse_stmt(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Stmt, usize> {
    if let Some(TokenType::Print) = iter.peek().map(|t| &t.token_type)  {
        return parse_print(&mut iter);
    }

    if let Some(TokenType::LeftBrace) = iter.peek().map(|t| &t.token_type)  {
        return parse_block(&mut iter);
    }

    parse_exprstmt(&mut iter)
}

fn parse_vardecl(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Stmt, usize> {
    if let Some(_) = match_next(&mut iter, &[TokenType::Var]) {
        if let Some(id) = iter.next() {
            match id {
                t
                @
                Token {
                    token_type: TokenType::Identifier(_),
                    ..
                } => {
                    let expr = if match_next(&mut iter, &[TokenType::Equal]).is_some() {
                        parse_expression(&mut iter)?
                    } else {
                        Expr::Leaf(Token {
                            token_type: TokenType::Nil,
                            line: t.line,
                        })
                    };
                    if match_next(&mut iter, &[TokenType::Semicolon]).is_none() {
                        Err(44)
                    } else {
                        Ok(Stmt::Var(t, expr))
                    }
                }
                _ => Err(32),
            }
        } else {
            Err(31)
        }
    } else {
        Err(30)
    }
}

fn parse_decl(mut iter: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Stmt, usize> {
    if iter.peek().map(|t| &t.token_type) == Some(&TokenType::Var) {
        parse_vardecl(&mut iter)
    }
    else {
        parse_stmt(&mut iter)
    }
}

pub fn parse<'a, I>(tokens: I) -> Vec<Stmt>
where
    I: IntoIterator<Item = Token>,
{
    let mut res = vec![];
    let mut iter = tokens.into_iter().peekable();
    while !iter.peek().is_none() {
        if let Ok(stmt) = parse_decl(&mut iter) {
            res.push(stmt);
        } else {
            iter.next(); // skip unparsable token
        }
    }
    res
}
