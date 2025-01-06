mod lexer_error;
mod token;
mod token_stream;

use lexer_error::*;
use token_stream::*;

pub use lexer_error::LexerError;
pub use token::{Span, Token, TokenType};

pub fn tokenise(source: &str) -> Result<Vec<Token>, LexerError> {
    TokenStream::new(source).tokenise()
}
