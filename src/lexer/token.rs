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
            TokenType::Ident(name) => write!(f, "identifier `{name}`"),
            TokenType::Num(num) => write!(f, "number literal `{num}`"),
            TokenType::Mul => write!(f, "`*`"),
            TokenType::Div => write!(f, "`/`"),
            TokenType::Add => write!(f, "`+`"),
            TokenType::Sub => write!(f, "`-`"),
            TokenType::And => write!(f, "`&&`"),
            TokenType::Or => write!(f, "`||`"),
            TokenType::Not => write!(f, "`!`"),
            TokenType::Eq => write!(f, "`==`"),
            TokenType::Neq => write!(f, "`!=`"),
            TokenType::Assign => write!(f, "`=`"),
            TokenType::Lambda => write!(f, "`\\`"),
            TokenType::Dot => write!(f, "`.`"),
            TokenType::LPar => write!(f, "`(`"),
            TokenType::RPar => write!(f, "`)`"),
            TokenType::Comment(text) => write!(f, "comment `{text}`"),
            TokenType::Eol => write!(f, "end of line"),
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
