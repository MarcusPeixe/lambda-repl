mod error;
mod parsed;
mod ast;

use crate::lexer;

use error::*;

pub fn parse<'src>(tokens: &lexer::TokenVec<'src>) -> ast::Node<'src> {
    let iter = tokens.iter();
    unimplemented!()
}
