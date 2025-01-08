mod lexer_error;
mod token;
mod token_stream;

use crate::source;

use lexer_error::*;
use token_stream::*;

pub use lexer_error::LexerError;
pub use token::{Span, Token, TokenType};

pub fn tokenise(source: &source::Source) -> LexerResult<Vec<Token>> {
    TokenStream::new(&source.text).tokenise()
}
