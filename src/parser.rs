use crate::token::{Token, TokenType};

use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Expr {
    Leaf(Token),
    // Variable(Token), // probably won't need this?
    Assign(String, Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Logical(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    While(Expr, Box<Stmt>),
}

struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    iter: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn match_next(&mut self, options: &[TokenType]) -> Option<Token> {
        for token_type in options {
            if self.iter.peek().map(|t| &t.token_type) == Some(&token_type) {
                return self.iter.next();
            };
        }
        None
    }

    fn parse_primary(&mut self) -> Result<Expr, usize> {
        if let Some(op) = self.match_next(&[TokenType::False, TokenType::True, TokenType::Nil]) {
            return Ok(Expr::Leaf(op));
        }

        if self.match_next(&[TokenType::LeftParen]).is_some() {
            let expr = self.parse_expression()?;

            if self.match_next(&[TokenType::RightParen]).is_none() {
                return Err(1); // Missing closing brace
            }

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        // Only options left are literals/variables
        if let Some(t) = self.iter.next() {
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

    fn parse_unary(&mut self) -> Result<Expr, usize> {
        if let Some(op) = self.match_next(&[TokenType::Bang, TokenType::Minus]) {
            let rhs = self.parse_unary()?;
            return Ok(Expr::Unary(op, Box::new(rhs)));
        }

        self.parse_primary()
    }

    fn parse_multiplication(&mut self) -> Result<Expr, usize> {
        let mut lhs = self.parse_unary()?;
        while let Some(op) = self.match_next(&[TokenType::Slash, TokenType::Star]) {
            let rhs = self.parse_unary()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_addition(&mut self) -> Result<Expr, usize> {
        let mut lhs = self.parse_multiplication()?;
        while let Some(op) = self.match_next(&[TokenType::Plus, TokenType::Minus]) {
            let rhs = self.parse_multiplication()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_comparison(&mut self) -> Result<Expr, usize> {
        let mut lhs = self.parse_addition()?;
        while let Some(op) = self.match_next(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let rhs = self.parse_addition()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_equality(&mut self) -> Result<Expr, usize> {
        let mut lhs = self.parse_comparison()?;
        while let Some(op) = self.match_next(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let rhs = self.parse_comparison()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_print(&mut self) -> Result<Stmt, usize> {
        let expr = self.parse_expression()?;
        if self.match_next(&[TokenType::Semicolon]).is_none() {
            Err(22)
        } else {
            Ok(Stmt::Print(expr))
        }
    }

    fn parse_logic_and(&mut self) -> Result<Expr, usize> {
        let mut expr = self.parse_equality()?;

        while let Some(op) = self.match_next(&[TokenType::And]) {
            let rhs = self.parse_equality()?;
            expr = Expr::Logical(op, Box::new(expr), Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_logic_or(&mut self) -> Result<Expr, usize> {
        let mut expr = self.parse_logic_and()?;

        while let Some(op) = self.match_next(&[TokenType::Or]) {
            let rhs = self.parse_logic_and()?;
            expr = Expr::Logical(op, Box::new(expr), Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_assignment(&mut self) -> Result<Expr, usize> {
        let expr = self.parse_logic_or()?;

        if self.match_next(&[TokenType::Equal]).is_some() {
            let rhs = self.parse_assignment()?;

            match expr {
                Expr::Leaf(Token {
                    token_type: TokenType::Identifier(s),
                    ..
                }) => Ok(Expr::Assign(s, Box::new(rhs))),
                _ => Err(42),
            }
        } else {
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

    fn parse_expression(&mut self) -> Result<Expr, usize> {
        self.parse_assignment()
    }

    fn parse_exprstmt(&mut self) -> Result<Stmt, usize> {
        let expr = self.parse_expression()?;
        if self.match_next(&[TokenType::Semicolon]).is_some() {
            Ok(Stmt::Expression(expr))
        } else {
            Err(22)
        }
    }

    fn parse_block(&mut self) -> Result<Stmt, usize> {
        let mut stmts = vec![];
        while self.iter.peek().is_some()
            && self.iter.peek().map(|t| &t.token_type) != Some(&TokenType::RightBrace)
        {
            stmts.push(self.parse_decl()?);
        }
        // drop right brace
        self.iter.next();
        Ok(Stmt::Block(stmts))
    }

    fn parse_if(&mut self) -> Result<Stmt, usize> {
        if self.match_next(&[TokenType::LeftParen]).is_none() {
            return Err(51);
        }

        let cond = self.parse_expression()?;

        if self.match_next(&[TokenType::RightParen]).is_none() {
            return Err(51);
        }

        let then_branch = self.parse_stmt()?;
        let else_branch = if self.match_next(&[TokenType::Else]).is_some() {
            Some(self.parse_stmt()?)
        } else {
            None
        };

        Ok(Stmt::If(cond, Box::new(then_branch), Box::new(else_branch)))
    }

    fn parse_while(&mut self) -> Result<Stmt, usize> {
        if self.match_next(&[TokenType::LeftParen]).is_none() {
            return Err(51);
        }

        let cond = self.parse_expression()?;

        if self.match_next(&[TokenType::RightParen]).is_none() {
            return Err(51);
        }
        let body = self.parse_stmt()?;

        Ok(Stmt::While(cond, Box::new(body)))
    }

    fn parse_for(&mut self) -> Result<Stmt, usize> {
        if self.match_next(&[TokenType::LeftParen]).is_none() {
            return Err(61);
        }

        let initializer = if self.match_next(&[TokenType::Semicolon]).is_some() {
            None
        } else if self.match_next(&[TokenType::Var]).is_some() {
            Some(self.parse_vardecl()?)
        } else {
            Some(self.parse_exprstmt()?)
        };

        let cond = if let Some(sc) = self.match_next(&[TokenType::Semicolon]) {
            Expr::Leaf(Token {
                token_type: TokenType::True,
                line: sc.line,
            })
        } else {
            self.parse_expression()?
        };

        if self.match_next(&[TokenType::Semicolon]).is_none() {
            return Err(62);
        }

        let increment = if self.iter.peek().map(|t| &t.token_type) == Some(&TokenType::RightParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        if self.match_next(&[TokenType::RightParen]).is_none() {
            return Err(63);
        }

        let mut body = self.parse_stmt()?;

        if let Some(incr) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(incr)]);
        };

        body = Stmt::While(cond, Box::new(body));

        if let Some(init) = initializer {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, usize> {
        if self.match_next(&[TokenType::For]).is_some() {
            return self.parse_for();
        }

        if self.match_next(&[TokenType::If]).is_some() {
            return self.parse_if();
        }

        if self.match_next(&[TokenType::Print]).is_some() {
            return self.parse_print();
        }

        if self.match_next(&[TokenType::While]).is_some() {
            return self.parse_while();
        }

        if self.match_next(&[TokenType::LeftBrace]).is_some() {
            return self.parse_block();
        }

        self.parse_exprstmt()
    }

    fn parse_vardecl(&mut self) -> Result<Stmt, usize> {
        if let Some(id) = self.iter.next() {
            match id {
                t
                @
                Token {
                    token_type: TokenType::Identifier(_),
                    ..
                } => {
                    let expr = if self.match_next(&[TokenType::Equal]).is_some() {
                        self.parse_expression()?
                    } else {
                        Expr::Leaf(Token {
                            token_type: TokenType::Nil,
                            line: t.line,
                        })
                    };
                    if self.match_next(&[TokenType::Semicolon]).is_none() {
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
    }

    fn parse_decl(&mut self) -> Result<Stmt, usize> {
        if self.match_next(&[TokenType::Var]).is_some() {
            self.parse_vardecl()
        } else {
            self.parse_stmt()
        }
    }
}

pub fn parse<'a, I>(tokens: I) -> Vec<Stmt>
where
    I: IntoIterator<Item = Token>,
{
    let mut parser = Parser {
        iter: tokens.into_iter().peekable(),
    };
    let mut res = vec![];
    while !parser.iter.peek().is_none() {
        if let Ok(stmt) = parser.parse_decl() {
            res.push(stmt);
        } else {
            parser.iter.next(); // skip unparsable token
        }
    }
    res
}
