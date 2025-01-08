mod lexer;
mod source;

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
    // let code = "(\\x.\\y.x) a b";
    let code = "(λx1.λx2.x1) a $ b1b";
    let source = source::Source::new(code.to_owned());
    // let tokens = lexer::tokenise(source).unwrap();
    // println!("Tokens:");
    // for token in tokens {
    //     println!("- {token}");
    // }
    let tokens = lexer::tokenise(&source);
    match tokens {
        Ok(tokens) => {
            println!("Tokens:");
            for token in tokens {
                println!("- {token}");
            }
        }
        Err(error) => {
            dbg!(error);
        }
    };
}

