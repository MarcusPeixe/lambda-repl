mod lexer;

enum AST<'a> {
    Var(&'a str),
    Num(u64),
    App(Box<AST<'a>>, Box<AST<'a>>),
    Mul(Box<AST<'a>>, Box<AST<'a>>),
    Div(Box<AST<'a>>, Box<AST<'a>>),
    Add(Box<AST<'a>>, Box<AST<'a>>),
    Sub(Box<AST<'a>>, Box<AST<'a>>),
    Abs(&'a str, Box<AST<'a>>),
}

fn main() {
    // let source = "(\\x.\\y.x) a b";
    let source = "(λx1.λx2.x1) a b1b";
    let tokens = lexer::tokenise(source).unwrap();
    println!("Tokens:");
    for token in tokens {
        println!("- {token}");
    }
}

