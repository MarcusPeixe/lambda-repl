use super::*;

type LexerInput<'src> = std::iter::Peekable<std::str::CharIndices<'src>>;

pub struct TokenStream<'src> {
    iter: LexerInput<'src>,
    tokens: TokenVec<'src>,
    errors: Vec<LexerError<'src>>,
}

impl<'src> TokenStream<'src> {
    pub fn new(source: &'src source::Source) -> Self {
        Self {
            iter: source.text.char_indices().peekable(),
            tokens: TokenVec {
                source,
                tokens: Vec::new(),
            },
            errors: Vec::new(),
        }
    }

    fn peek(&mut self) -> (usize, Option<char>) {
        self.iter
            .peek()
            .map(|&(idx, ch)| (idx, Some(ch)))
            .unwrap_or((self.tokens.source.text.len(), None))
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

    fn invalid_token(&mut self, start: usize) {
        while let (_index, Some(ch)) = self.peek() {
            if ch.is_alphanumeric() || ch.is_whitespace() {
                break;
            }
            self.next();
        }
        let end = self.peek().0;
        let span = Span::new(&self.tokens.source.text, start, end);
        self.errors
            .push(LexerError::invalid_token(self.tokens.source, span));
    }

    fn invalid_number(&mut self, span: Span<'src>, error: std::num::ParseIntError) {
        self.errors.push(LexerError::invalid_number(
            self.tokens.source,
            span,
            &error.to_string(),
        ));
    }

    fn push_symbol<'orig>(&mut self, start: usize)
    where
        'src: 'orig,
    {
        let source = &self.tokens.source.text[start..];
        let possible_symbols = [
            ("(", 1, TokenType::LPar),
            (")", 1, TokenType::RPar),
            ("\\", 1, TokenType::Lambda),
            ("λ", 1, TokenType::Lambda),
            (".", 1, TokenType::Dot),
            ("->", 2, TokenType::Dot),
            ("==", 2, TokenType::Eq),
            ("=", 1, TokenType::Assign),
            ("+", 1, TokenType::Add),
            ("-", 1, TokenType::Sub),
            ("*", 1, TokenType::Mul),
            ("/", 1, TokenType::Div),
            ("!=", 2, TokenType::Neq),
            ("!", 1, TokenType::Not),
            ("&&", 2, TokenType::And),
            ("||", 2, TokenType::Or),
        ];
        for (symbol, size, token_type) in possible_symbols {
            if source.starts_with(symbol) {
                let end = start + symbol.len();
                let span = Span::new(&self.tokens.source.text, start, end);
                self.tokens.tokens.push(Token::new(token_type, span));
                self.skip(size);
                return;
            }
        }
        self.invalid_token(start);
    }

    fn push_variable(&mut self, start: usize) {
        let end = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        let span = Span::new(&self.tokens.source.text, start, end);
        let var_name = span.get_text(&self.tokens.source.text);
        self.tokens
            .tokens
            .push(Token::new(TokenType::Var(var_name), span));
    }

    fn push_number<'orig>(&mut self, start: usize)
    where
        'src: 'orig,
    {
        let end = self.consume_while(char::is_alphanumeric);
        let span = Span::new(&self.tokens.source.text, start, end);
        let number = match span.get_text(&self.tokens.source.text).parse() {
            Ok(number) => number,
            Err(error) => {
                self.invalid_number(span, error);
                return;
            }
        };
        self.tokens
            .tokens
            .push(Token::new(TokenType::Num(number), span));
    }

    pub fn tokenise<'orig>(mut self) -> LexerResult<'orig, TokenVec<'orig>>
    where
        'src: 'orig,
    {
        while let (index, Some(ch)) = self.peek() {
            match ch {
                ch if "λ\\*/+().&|!-=".contains(ch) => self.push_symbol(index),
                ch if ch.is_whitespace() => self.skip_whitespace(),
                ch if ch.is_alphabetic() => self.push_variable(index),
                ch if ch.is_numeric() => self.push_number(index),
                _ => self.invalid_token(index),
            }
        }
        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(LexerErrorVec {
                tokens: self.tokens,
                errors: self.errors,
            })
        }
    }
}

#[derive(Debug)]
pub struct TokenVec<'t> {
    pub source: &'t source::Source,
    pub tokens: Vec<Token<'t>>,
}

