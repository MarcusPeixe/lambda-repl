mod error;
mod parser_state;
mod ast;

use crate::lexer;

use error::*;

pub fn parse_file<'src>(tokens: &'src lexer::TokenVec<'src>) -> ParserResult<'src> {
    parser_state::ParserState::new(tokens).parse_file()
}

pub fn parse_line<'src>(tokens: &'src lexer::TokenVec<'src>) -> ParserResult<'src> {
    parser_state::ParserState::new(tokens).parse_line()
}
