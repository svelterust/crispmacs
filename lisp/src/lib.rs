#![feature(if_let_guard)]

mod parse;
mod eval;

pub use parse::parse;
pub use eval::Context;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    Not,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(i32),
    Keyword(String),
    Boolean(bool),
    BuiltIn(BuiltIn),
    Symbol(String),
}

// Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    /// (func-name arg1 arg2 arg3 ...)
    Application(Box<Expr>, Vec<Expr>),
    /// (if predicate do-this)
    If(Box<Expr>, Box<Expr>),
    /// (if predicate do-this otherwise-do-this)
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    /// '(3 (if (+ 3 3) 4 5) 7)
    Quote(Vec<Expr>),
    /// (define red 123)
    Define(Atom, Box<Expr>),
    /// (lambda (x y z) (+ x y z))
    Lambda(Vec<Expr>, Box<Expr>),
}

pub fn parse_and_eval(input: &str) -> Result<Vec<Expr>, nom_supreme::error::ErrorTree<&str>> {
    let mut context = eval::Context::default();
    parse::parse(input).map(|items| items.into_iter().filter_map(|it| context.eval(it)).collect::<Vec<_>>())
}
