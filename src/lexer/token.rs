use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'src> {
    Ident(&'src str),
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
    Comment(&'src str),
    Eol,
}

impl std::fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Ident(name) => write!(f, "Var({name})"),
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
            TokenType::Comment(text) => write!(f, "Comment({text})"),
            TokenType::Eol => write!(f, "EOL"),
        }
    }
}

pub struct Token<'src> {
    pub token_type: TokenType<'src>,
    pub span: Span<'src>,
}

impl<'src> Token<'src> {
    pub fn new(token_type: TokenType<'src>, span: Span<'src>) -> Self {
        Self { token_type, span }
    }
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {:?}>", self.token_type, self.span)
    }
}
