use super::*;

#[derive(Debug)]
pub struct LexerError<'src> {
    span: Span<'src>,
    message: String,
}

impl<'src> LexerError<'src> {
    pub fn invalid_number(source: &'src source::Source, span: Span<'src>, details: &str) -> Self {
        let literal = span.get_text(&source.text);
        Self {
            span,
            message: format!("invalid number literal `{literal}` ({details})"),
        }
    }

    pub fn invalid_token(source: &'src source::Source, span: Span<'src>) -> Self {
        let literal = span.get_text(&source.text);
        Self {
            span,
            message: format!("invalid token `{literal}`"),
        }
    }

    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, tokens: &TokenVec) -> std::fmt::Result {
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
}

pub type LexerResult<'src, T> = Result<T, LexerErrorVec<'src>>;

#[derive(Debug)]
pub struct LexerErrorVec<'src> {
    pub tokens: TokenVec<'src>,
    pub errors: Vec<LexerError<'src>>,
}

impl std::fmt::Display for LexerErrorVec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            error.print(f, &self.tokens)?;
        }
        Ok(())
    }
}
