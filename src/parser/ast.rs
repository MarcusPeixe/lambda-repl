pub type Node<'src> = Box<Ast<'src>>;

#[derive(Debug)]
pub enum Ast<'src> {
    Var(&'src str),
    Num(u64),
    Not(Node<'src>),
    App(Node<'src>, Node<'src>),
    Mul(Node<'src>, Node<'src>),
    Div(Node<'src>, Node<'src>),
    Add(Node<'src>, Node<'src>),
    Sub(Node<'src>, Node<'src>),
    And(Node<'src>, Node<'src>),
    Or(Node<'src>, Node<'src>),
    Eq(Node<'src>, Node<'src>),
    Neq(Node<'src>, Node<'src>),
    Abs(&'src str, Node<'src>),
    Assign(&'src str, Node<'src>),
    Source(Vec<Ast<'src>>),
}
