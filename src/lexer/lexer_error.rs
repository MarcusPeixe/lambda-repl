use super::*;

#[derive(Debug)]
pub struct LexerError<'src> {
    source: &'src str,
    error: Option<Box<dyn std::error::Error>>,
    span: Span<'src>,
    message: String,
}

impl<'src> LexerError<'src> {
    pub fn invalid_number(source: &'src str, span: Span<'src>, error: std::num::ParseIntError) -> Self {
        let literal = span.get_text(source);
        Self {
            source,
            error: Some(Box::new(error)),
            span,
            message: format!("invalid number literal \"{literal}\""),
        }
    }

    pub fn invalid_symbol(source: &'src str, span: Span<'src>) -> Self {
        let literal = span.get_text(source);
        Self {
            source,
            error: None,
            span,
            message: format!("invalid token \"{literal}\""),
        }
    }
}

impl<'src> std::fmt::Display for LexerError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lexer error: {}", self.message)
    }
}

impl<'src> std::error::Error for LexerError<'src> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.as_deref()
    }
}

pub type LexerResult<'src, T> = Result<T, LexerError<'src>>;
