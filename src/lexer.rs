#[derive(Debug)]
pub enum TokenType<'src> {
    Var(&'src str),
    Num(u64),
    Mul,
    Div,
    Add,
    Sub,
    And,
    Or,
    Not,
    Eq,
    Neq,
    Assign,
    Lambda,
    Dot,
    LPar,
    RPar,
}

#[derive(Debug)]
pub struct Span<'src> {
    start: usize,
    end: usize,
    source: std::marker::PhantomData<&'src str>,
}

impl<'src> Span<'src> {
    fn new(_source: &'src str, start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            source: std::marker::PhantomData::<&'src str>,
        }
    }

    fn get_text(&self, source: &'src str) -> &'src str {
        &source[self.start..self.end]
    }
}

#[derive(Debug)]
pub struct Token<'src> {
    token_type: TokenType<'src>,
    span: Span<'src>,
}

impl<'src> Token<'src> {
    fn new(token_type: TokenType<'src>, span: Span<'src>) -> Self {
        Self { token_type, span }
    }
}

#[derive(Debug)]
pub struct LexerError<'src> {
    source: &'src str,
    error: Option<Box<dyn std::error::Error>>,
    span: Span<'src>,
    message: String,
}

impl<'src> LexerError<'src> {
    fn invalid_number(source: &'src str, span: Span<'src>, error: std::num::ParseIntError) -> Self {
        let literal = span.get_text(source);
        Self {
            source,
            error: Some(Box::new(error)),
            span,
            message: format!("invalid number literal \"{literal}\""),
        }
    }

    fn invalid_symbol(source: &'src str, span: Span<'src>) -> Self {
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

type LexerResult<'src, T> = Result<T, LexerError<'src>>;

type LexerInput<'src> = std::iter::Peekable<std::str::CharIndices<'src>>;

struct TokenStream<'src> {
    source: &'src str,
    iter: LexerInput<'src>,
    tokens: Vec<Token<'src>>,
}

impl<'src> TokenStream<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            iter: source.char_indices().peekable(),
            tokens: Vec::new(),
        }
    }

    fn peek(&mut self) -> (usize, Option<char>) {
        self.iter
            .peek()
            .map(|&(idx, ch)| (idx, Some(ch)))
            .unwrap_or((self.source.len(), None))
    }

    fn next(&mut self) {
        self.iter.next();
    }

    fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.iter.next();
        }
    }

    fn skip_whitespace(&mut self) {
        while let (_idx, Some(char)) = self.peek() {
            if !char.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    fn consume_while(&mut self, pred: fn(char) -> bool) -> usize {
        while let (_index, Some(char)) = self.peek() {
            if !pred(char) {
                break;
            }
            self.next();
        }
        self.peek().0
    }

    fn push_symbol(&mut self, start: usize, token_type: TokenType<'src>) {
        self.next();
        let end = self.peek().0;
        let span = Span::new(self.source, start, end);
        self.tokens.push(Token::new(token_type, span));
    }

    fn invalid_symbol<'orig>(&self, start: usize) -> LexerError<'orig>
    where
        'src: 'orig,
    {
        let end = start + 1;
        let span = Span::new(self.source, start, end);
        LexerError::invalid_symbol(self.source, span)
    }

    fn push_long_symbol<'orig>(&mut self, start: usize) -> LexerResult<'orig, ()>
    where
        'src: 'orig,
    {
        let source = &self.source[start..];
        let possible_symbols = [
            ("&&", TokenType::And),
            ("||", TokenType::Or),
            ("!=", TokenType::Neq),
            ("!", TokenType::Not),
            ("==", TokenType::Eq),
            ("=", TokenType::Assign),
            ("->", TokenType::Dot),
            ("-", TokenType::Sub),
        ];
        for (symbol, token_type) in possible_symbols {
            if source.starts_with(symbol) {
                let end = start + symbol.len();
                let span = Span::new(self.source, start, end);
                self.tokens.push(Token::new(token_type, span));
                self.skip(symbol.len());
                return Ok(());
            }
        }
        Err(self.invalid_symbol(start))
    }

    fn push_variable(&mut self, start: usize) {
        let end = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        let span = Span::new(self.source, start, end);
        let var_name = span.get_text(self.source);
        self.tokens.push(Token::new(TokenType::Var(var_name), span));
    }

    fn push_number<'orig>(&mut self, start: usize) -> LexerResult<'orig, ()>
    where
        'src: 'orig,
    {
        let end = self.consume_while(char::is_alphanumeric);
        let span = Span::new(self.source, start, end);
        let number = match span.get_text(self.source).parse() {
            Ok(number) => number,
            Err(error) => {
                return Err(LexerError::invalid_number(self.source, span, error));
            }
        };
        self.tokens.push(Token::new(TokenType::Num(number), span));
        Ok(())
    }

    fn tokenise<'orig>(&mut self) -> LexerResult<'orig, ()>
    where
        'src: 'orig,
    {
        while let (index, Some(char)) = self.peek() {
            match char {
                'λ' | '\\' => self.push_symbol(index, TokenType::Lambda),

                '*' => self.push_symbol(index, TokenType::Mul),
                '/' => self.push_symbol(index, TokenType::Div),
                '+' => self.push_symbol(index, TokenType::Add),
                '(' => self.push_symbol(index, TokenType::LPar),
                ')' => self.push_symbol(index, TokenType::RPar),
                '.' => self.push_symbol(index, TokenType::Dot),

                '&' | '|' | '!' | '=' | '-' => self.push_long_symbol(index)?,

                ch if ch.is_numeric() => self.push_number(index)?,
                ch if ch.is_alphabetic() => self.push_variable(index),
                ch if ch.is_whitespace() => self.skip_whitespace(),

                _ => return Err(self.invalid_symbol(index)),
            }
        }
        Ok(())
    }
}

pub fn tokenise(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut stream = TokenStream::new(source);
    stream.tokenise()?;
    Ok(stream.tokens)
}
