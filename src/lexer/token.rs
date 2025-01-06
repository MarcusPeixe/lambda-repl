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

impl<'src> std::fmt::Display for TokenType<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Var(name) => write!(f, "Var({name})"),
            TokenType::Num(num) => write!(f, "Num({num})"),
            TokenType::Mul => write!(f, "Mul"),
            TokenType::Div => write!(f, "Div"),
            TokenType::Add => write!(f, "Add"),
            TokenType::Sub => write!(f, "Sub"),
            TokenType::And => write!(f, "And"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Not => write!(f, "Not"),
            TokenType::Eq => write!(f, "Eq"),
            TokenType::Neq => write!(f, "Neq"),
            TokenType::Assign => write!(f, "Assign"),
            TokenType::Lambda => write!(f, "Lambda"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::LPar => write!(f, "LPar"),
            TokenType::RPar => write!(f, "RPar"),
        }
    }
}

#[derive(Debug)]
pub struct Span<'src> {
    start: usize,
    end: usize,
    source: std::marker::PhantomData<&'src str>,
}

impl<'src> std::fmt::Display for Span<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}..{}>", self.start, self.end)
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
}

#[derive(Debug)]
pub struct Token<'src> {
    token_type: TokenType<'src>,
    span: Span<'src>,
}

impl<'src> Token<'src> {
    pub fn new(token_type: TokenType<'src>, span: Span<'src>) -> Self {
        Self { token_type, span }
    }
}

impl<'src> std::fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.token_type, self.span)
    }
}

