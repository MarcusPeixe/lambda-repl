use super::*;

pub struct ParserErrorNote<'src> {
    message: Option<String>,
    span: lexer::Span<'src>,
}

pub struct ParserError<'src> {
    message: String,
    notes: Vec<ParserErrorNote<'src>>,
}