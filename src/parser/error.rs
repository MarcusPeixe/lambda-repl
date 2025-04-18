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

    pub fn new_end(message: String, tokens: &'src lexer::TokenVec) -> Self {
        let end = tokens.source.text.len();
        let span = lexer::Span::new(&tokens.source.text, end, end);
        Self { message, span }
    }

    pub fn new(message: String, tokens: &'src lexer::TokenVec, start: usize, end: usize) -> Self {
        let span = lexer::Span::new(&tokens.source.text, start, end);
        Self { message, span }
    }
}

pub type ParserResult<'src> = Result<ast::Node<'src>, ParserErrorVec<'src>>;

#[derive(Debug)]
pub struct ParserErrorVec<'src> {
    pub tokens: &'src lexer::TokenVec<'src>,
    pub errors: Vec<ParserError<'src>>,
}

impl<'src> ParserErrorVec<'src> {
    pub fn new(tokens: &'src lexer::TokenVec) -> Self {
        Self {
            tokens,
            errors: Vec::new(),
        }
    }

    pub fn single(
        message: String,
        tokens: &'src lexer::TokenVec,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            tokens,
            errors: vec![ParserError::new(message, tokens, start, end)],
        }
    }

    pub fn single_end(message: String, tokens: &'src lexer::TokenVec) -> Self {
        Self {
            tokens,
            errors: vec![ParserError::new_end(message, tokens)],
        }
    }

    pub fn get_longer_of(self, other: Self) -> Self {
        let other_is_longer = other.errors.len() > self.errors.len()
            || other.errors.len() == self.errors.len()
                && !self.errors.is_empty()
                && other.errors[0].span.start > self.errors[0].span.start;
        if other_is_longer {
            other
        } else {
            self
        }
    }

    pub fn combine(&mut self, other: Self) {
        self.errors.extend(other.errors);
    }
}

impl std::fmt::Display for ParserErrorVec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            error.print(f, self.tokens)?;
        }
        Ok(())
    }
}
