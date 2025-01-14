mod error;
mod span;
mod token;
mod token_vec;

use crate::source;

use error::*;
use token_vec::*;

pub use error::LexerError;
pub use span::Span;
pub use token::{Token, TokenType};
pub use token_vec::{TokenVec, TokenIter};

pub fn tokenise(source: &source::Source) -> LexerResult<TokenVec> {
    TokenStream::new(source).tokenise()
}
