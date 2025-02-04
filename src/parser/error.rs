use super::*;

#[derive(Debug)]
pub struct ParserError<'src> {
    message: String,
    span: lexer::Span<'src>,
}

impl<'src> ParserError<'src> {
    fn print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tokens: &'src lexer::TokenVec,
    ) -> std::fmt::Result {
        writeln!(f, "\x1B[1;31mParser error:\x1B[39m {}\x1B[m", self.message)?;
        writeln!(
            f,
            "   \x1B[1;34m-->\x1B[m {}",
            self.span.get_location_str(tokens.source)
        )?;
        writeln!(f, "    \x1B[1;34m|\x1B[m")?;
        self.span.print(f, tokens)?;
        writeln!(f)
    }

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
        Self { message, span }
    }

    pub fn new(message: String, tokens: &'src lexer::TokenVec, start: usize, end: usize) -> Self {
        let span = lexer::Span::new(&tokens.source.text, start, end);
        Self { message, span }
    }

    pub fn expect(self, message: String) -> Result<ast::Node<'src>, ParserError<'src>> {
        Err(ParserError {
            message,
            span: self.span,
        })
    }

    pub fn to_singleton(self, tokens: &'src lexer::TokenVec) -> ParserErrorVec<'src> {
        ParserErrorVec {
            tokens,
            errors: vec![self],
        }
    }
}

pub type ParserResult<'src> = Result<ast::Node<'src>, ParserErrorVec<'src>>;

#[derive(Debug)]
pub struct ParserErrorVec<'src> {
    pub tokens: &'src lexer::TokenVec<'src>,
    pub errors: Vec<ParserError<'src>>,
}

impl<'src> std::fmt::Display for ParserErrorVec<'src> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for error in &self.errors {
            error.print(f, self.tokens)?;
        }
        Ok(())
    }
}
