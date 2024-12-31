enum ASTNode<'a> {
    Var(&'a str),
    Num(u64),
    App(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    Mul(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    Div(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    Add(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    Sub(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    Abs(&'a str, Box<ASTNode<'a>>),
}

type ParserInput<'a> = std::iter::Peekable<std::str::Chars<'a>>;

struct ParserError<'a>(&'a str, String);

type ParserResult<'a, T> = Result<(T, ParserInput<'a>), ParserError<'a>>;

fn parse_whitespace(mut input: ParserInput) -> ParserResult<()> {
    while let Some(&ch) = input.peek() {
        if !ch.is_whitespace() {
            break;
        }
    }
    Ok(((), input))
}

// fn parse_identifier(mut input: ParserInput) -> ParserResult<>

fn main() {
    println!("Hello, world!");
}
