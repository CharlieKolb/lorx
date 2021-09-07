use crate::interpreter::Interpreter;
use crate::environment::EnvStack;
use crate::parser::{ Stmt, Expr, StmtFunction };
use crate::value::{ Value, Er };

#[derive(Debug, Default)]
struct Resolver {
    interpreter: Interpreter,
    env: EnvStack::<bool>,
}

impl Resolver {
    fn resolve_block(&mut self, stmts: &Vec<Stmt>) {
        self.env.push_default();
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
        self.env.pop().unwrap();
    }

    fn resolve_fun(&mut self, stmtFun: &StmtFunction) {
        self.env.push_default();
        for param in &stmtFun.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_block(&stmtFun.body);
        self.env.pop().unwrap();
    }

    fn resolve_if(&mut self, cond: &Expr, then: &Stmt, els: &Option<Stmt>) {
        self.resolve_expr(cond);
        self.resolve_stmt(then);
        if let Some(els_stmt) = els {
            self.resolve_stmt(els_stmt);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Assign(name, exp) => {
                self.resolve_expr(exp);
                if self.env.get(name).ok().copied() == Some(false) {
                    // todo proper logging
                    println!("Cannot read local variable in its own initializer");
                }

                if let Some(depth) = self.env.resolve_depth(name) {
                    self.interpreter.resolve(exp, depth)
                };
            }
            Expr::Binary(_, l, r) | Expr::Logical(_, l, r) => {
                self.resolve_expr(l);
                self.resolve_expr(r);
            }
            Expr::Call(_, callee, args) => {
                self.resolve_expr(callee);
                for expr in args {
                    self.resolve_expr(expr);
                }
            }
            Expr::Grouping(expr) | Expr::Unary(_, expr) => {
                self.resolve_expr(expr);
            }
            Expr::Leaf(_) => (),
        }
    }

    // note that declarations on the top level noop for the resolver
    fn declare(&mut self, name: &String) {
        self.env.define(name, false);
    }

    fn define(&mut self, name: &String) {
        self.env.assign(name, true).ok();
    }

    fn resolve_var(&mut self, name: &String, expr: &Expr) {
        self.declare(name);
        self.resolve_expr(expr);
        self.define(name);
    }

    fn resolve_while(&mut self, cond: &Expr, body: &Stmt) {
        self.resolve_expr(cond);
        self.resolve_stmt(body);
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(stmts) => self.resolve_block(stmts),
            Stmt::Function(stmt_fun) => self.resolve_fun(stmt_fun),
            Stmt::If(cond, then, els) => self.resolve_if(cond, &*then, &*els),
            | Stmt::Expression(expr)
            | Stmt::Print(expr)
            | Stmt::Return(expr) => self.resolve_expr(expr),
            Stmt::Var(name, expr) => self.resolve_var(name, expr),
            Stmt::While(cond, body) => self.resolve_while(cond, body),
            _ => unimplemented!(),
        }
    }
}