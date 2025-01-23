use super::*;

pub struct ParserError<'src> {
    message: String,
    span: lexer::Span<'src>,
}

pub type ParserResult<'src> = Result<ast::Node<'src>, ParserError<'src>>;

impl<'src> ParserError<'src> {
    pub fn print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tokens: &lexer::TokenVec,
    ) -> std::fmt::Result {
        writeln!(f, "\x1B[1;31mLexer error:\x1B[39m {}\x1B[m", self.message)?;
        writeln!(
            f,
            "   \x1B[1;34m-->\x1B[m {}",
            self.span.get_location_str(tokens.source)
        )?;
        writeln!(f, "    \x1B[1;34m|\x1B[m")?;
        self.span.print(f, tokens)?;
        writeln!(f)
    }

    pub fn new_end(message: String, tokens: &'src lexer::TokenVec) -> Self {
        let end = tokens.source.text.len();
        let span = lexer::Span::new(&tokens.source.text, end, end);
        Self { message, span }
    }

    pub fn new(message: String, tokens: &'src lexer::TokenVec, start: usize, end: usize) -> Self {
        let span = lexer::Span::new(&tokens.source.text, start, end);
        Self { message, span }
    }

    pub fn expect(self, message: String) -> ParserResult<'src> {
        Err(ParserError {
            message,
            span: self.span,
        })
    }
}
