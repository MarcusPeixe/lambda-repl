use super::*;

#[derive(Clone, Copy)]
pub struct Span<'src> {
    start: usize,
    end: usize,
    source: std::marker::PhantomData<&'src str>,
}

impl std::fmt::Debug for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}

impl<'src> Span<'src> {
    pub fn new(_source: &'src str, start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            source: std::marker::PhantomData::<&'src str>,
        }
    }

    pub fn get_text(&self, source: &'src str) -> &'src str {
        &source[self.start..self.end]
    }

    pub fn get_location_str(&self, source: &source::Source) -> String {
        let file = &source.file_name;
        let (line, column) = source.get_line_column(self.start);
        let line = line + 1;
        let column = column + 1;

        format!("{file}:{line}:{column}:")
    }

    pub fn print<'t>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tokens: &'t TokenVec<'t>,
    ) -> std::fmt::Result {
        let line_start = tokens.source.get_line(self.start);
        let line_end = tokens.source.get_line(self.end);

        for i in line_start..=line_end {
            print_line(f, tokens, i, self.start, self.end)?;
        }

        Ok(())
    }
}

fn print_line(
    f: &mut std::fmt::Formatter<'_>,
    tokens: &TokenVec<'_>,
    line: usize,
    start: usize,
    end: usize,
) -> std::fmt::Result {
    // let start_offset = tokens.source.lines[line];
    // let end_offset = *tokens
    //     .source
    //     .lines
    //     .get(line + 1)
    //     .unwrap_or(&tokens.source.text.len());

    let (start_offset, end_offset) = tokens.source.get_line_offset(line);

    write!(f, "\x1B[1;34m{:3} |\x1B[m  ", line + 1)?;

    let mut curr_token = tokens
        .tokens
        .binary_search_by(|t| {
            if t.span.end <= start_offset {
                std::cmp::Ordering::Less
            } else if t.span.start >= start_offset {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .unwrap_or_else(|p| p);

    let mut curr_color = "";
    let slice = &tokens.source.text[start_offset..end_offset].trim_end();

    let mut start_chars = 0;
    let mut end_chars = slice.len();

    for (i_chars, (i, c)) in slice.char_indices().enumerate() {
        let i = i + start_offset;
        let mut new_color = "";
        if let Some(token) = tokens.tokens.get(curr_token) {
            if i >= token.span.end {
                curr_token += 1;
            }
        }
        if curr_token < tokens.tokens.len() {
            let span_start = tokens.tokens[curr_token].span.start;
            let span_end = tokens.tokens[curr_token].span.end;
            if i >= span_start && i < span_end {
                new_color = match tokens.tokens[curr_token].token_type {
                    TokenType::Var(_) => ";38;5;153",
                    TokenType::Num(_) => ";38;5;133",
                    TokenType::Mul
                    | TokenType::Div
                    | TokenType::Add
                    | TokenType::Sub
                    | TokenType::And
                    | TokenType::Or
                    | TokenType::Not
                    | TokenType::Eq
                    | TokenType::Neq
                    | TokenType::Assign
                    | TokenType::LPar
                    | TokenType::RPar => ";38;5;133",
                    TokenType::Lambda | TokenType::Dot => ";1;38;5;215",
                };
            }
        }
        if i >= start && i < end {
            new_color = ";1;31";
        }
        if new_color != curr_color {
            write!(f, "\x1B[{}m", new_color)?;
            curr_color = new_color;
        }
        write!(f, "{}", c)?;

        if i == start {
            start_chars = i_chars;
        }
        if i == end {
            end_chars = i_chars;
        }
    }

    writeln!(f, "\x1B[m")?;
    writeln!(
        f,
        "\x1B[1;34m    |\x1B[m  {}\x1B[1;31m{}\x1B[m",
        " ".repeat(start_chars),
        "^".repeat(std::cmp::max(end_chars - start_chars, 1)),
    )
}
