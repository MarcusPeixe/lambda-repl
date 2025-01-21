use super::*;

#[derive(Debug, Clone, Copy)]
struct Flags {
    ignore_newline: bool,
}

struct Parser<'src> {
    token_vec: &'src lexer::TokenVec<'src>,
    errors: Vec<ParserError<'src>>,
    iter: lexer::TokenIter<'src>,
    stack: Vec<lexer::TokenIter<'src>>,
}

impl<'src> Parser<'src> {
    fn next(&mut self) -> Option<&'src lexer::Token<'src>> {
        self.iter.next()
    }

    fn peek(&mut self) -> Option<&'src lexer::Token<'src>> {
        self.iter.peek().copied()
    }

    fn push(&mut self) {
        self.stack.push(self.iter.clone());
    }

    fn pop(&mut self) {
        self.iter = self.stack.pop().expect("Error! Stack underflow");
    }

    fn parse_assignment(&mut self) -> ParserResult<'src> {
        let Some(ident) = self.next() else {
            return ParserError::new_end(
                "expected identifier".to_owned(),
                self.token_vec,
            );
        };
        unimplemented!();
    } 
}


