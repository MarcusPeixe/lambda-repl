mod error;
mod parser_state;
mod ast;

use crate::lexer;

use error::*;

pub fn parse<'src>(tokens: &'src lexer::TokenVec<'src>) -> Result<Vec<ast::Ast<'src>>, ParserError<'src>> {
    parser_state::ParserState::new(tokens).parse_file()
}
