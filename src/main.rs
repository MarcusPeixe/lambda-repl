enum ASTNode<'a> {
    Variable(&'a str),
    Application(Box<ASTNode<'a>>, Box<ASTNode<'a>>),
    Abstraction(&'a str, Box<ASTNode<'a>>),
}

fn main() {
    println!("Hello, world!");
}
