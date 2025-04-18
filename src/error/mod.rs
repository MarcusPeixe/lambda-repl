use super::*;

pub struct Error<'src> {
    message: String,
    span: lexer::Span<'src>,
}

impl<'src> Error<'src> {
    pub fn print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tokens: &lexer::TokenVec<'src>,
    ) -> std::fmt::Result {
        writeln!(f, "\x1B[1;31mError:\x1B[39m {}\x1B[m", self.message)?;
        writeln!(
            f,
            "   \x1B[1;34m-->\x1B[m {}",
            self.span.get_location_str(tokens.source)
        )?;
        writeln!(f, "    \x1B[1;34m|\x1B[m")?;
        self.span.print(f, tokens)?;
        writeln!(f)
    }
}

pub struct Errors<'src> {
    tokens: lexer::TokenVec<'src>,
    errors: Vec<Error<'src>>,
}

impl<'src> Errors<'src> {
    pub fn new(tokens: lexer::TokenVec<'src>) -> Self {
        Self {
            tokens,
            errors: Vec::new(),
        }
    }

    pub fn invalid_number(&mut self, span: lexer::Span<'src>, details: &str) {
        let literal = span.get_text(&self.tokens.source.text);
        self.errors.push(Error {
            span,
            message: format!("invalid number literal `{literal}` ({details})"),
        });
    }

    pub fn invalid_token(&mut self, span: lexer::Span<'src>) {
        let literal = span.get_text(&self.tokens.source.text);
        self.errors.push(Error {
            span,
            message: format!("invalid token `{literal}`"),
        });
    }
}

impl std::fmt::Display for Errors<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            error.print(f, &self.tokens)?;
        }
        Ok(())
    }
}
