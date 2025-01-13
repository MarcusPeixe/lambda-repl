mod lexer_error;
mod span;
mod token;
mod token_vec;

use crate::source;

use lexer_error::*;
use token_vec::*;

pub use lexer_error::LexerError;
pub use span::Span;
pub use token::{Token, TokenType};
pub use token_vec::TokenVec;

pub fn tokenise(source: &source::Source) -> LexerResult<TokenVec> {
    TokenStream::new(source).tokenise()
}
