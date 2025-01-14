mod error;

use crate::lexer;

use error::*;

type Node<'src> = Box<AST<'src>>;

enum AST<'src> {
    Var(&'src str),
    Num(u64),
    Not(Node<'src>),
    App(Node<'src>, Node<'src>),
    Mul(Node<'src>, Node<'src>),
    Div(Node<'src>, Node<'src>),
    Add(Node<'src>, Node<'src>),
    Sub(Node<'src>, Node<'src>),
    And(Node<'src>, Node<'src>),
    Or(Node<'src>, Node<'src>),
    Eq(Node<'src>, Node<'src>),
    Neq(Node<'src>, Node<'src>),
    Abs(&'src str, Node<'src>),
    Assign(&'src str, Node<'src>),
}

type ParserResult<'src> = Result<Node<'src>, ParserError<'src>>;

fn parse_assignment(iter: lexer::TokenIter) {

}

pub fn parse(tokens: &lexer::TokenVec) {
    let iter = tokens.iter();
    unimplemented!()
}