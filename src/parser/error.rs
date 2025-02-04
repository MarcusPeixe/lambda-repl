use super::*;

#[derive(Debug)]
pub struct ParserError<'src> {
    message: String,
    span: lexer::Span<'src>,
    tokens: &'src lexer::TokenVec<'src>,
}

pub type ParserResult<'src> = Result<ast::Node<'src>, ParserError<'src>>;

impl<'src> std::fmt::Display for ParserError<'src> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        writeln!(f, "\x1B[1;31mParser error:\x1B[39m {}\x1B[m", self.message)?;
        writeln!(
            f,
            "   \x1B[1;34m-->\x1B[m {}",
            self.span.get_location_str(self.tokens.source)
        )?;
        writeln!(f, "    \x1B[1;34m|\x1B[m")?;
        self.span.print(f, self.tokens)?;
        writeln!(f)
    }
}

impl<'src> ParserError<'src> {
    pub fn get_longer_of(first: Self, second: Self) -> Self {
        if first.span.start >= second.span.start {
            first
        } else {
            second
        }
    }

    pub fn new_end(message: String, tokens: &'src lexer::TokenVec) -> Self {
        let end = tokens.source.text.len();
        let span = lexer::Span::new(&tokens.source.text, end, end);
        Self { message, span, tokens }
    }

    pub fn new(message: String, tokens: &'src lexer::TokenVec, start: usize, end: usize) -> Self {
        let span = lexer::Span::new(&tokens.source.text, start, end);
        Self { message, span, tokens }
    }

    pub fn expect(self, message: String) -> ParserResult<'src> {
        Err(ParserError {
            message,
            span: self.span,
            tokens: self.tokens,
        })
    }
}
