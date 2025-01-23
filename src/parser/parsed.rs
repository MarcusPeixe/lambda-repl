use super::*;

#[derive(Debug, Clone, Copy)]
struct Flags {
    ignore_newline: bool,
}

struct ParserState<'src> {
    token_vec: &'src lexer::TokenVec<'src>,
    errors: Vec<ParserError<'src>>,
    iter: lexer::TokenIter<'src>,
    stack: Vec<lexer::TokenIter<'src>>,
}

impl<'src> ParserState<'src> {
    fn peek(&mut self) -> Option<&'src lexer::Token<'src>> {
        self.iter.peek().copied()
    }

    fn next(&mut self) -> Option<&'src lexer::Token<'src>> {
        self.iter.next()
    }

    fn push(&mut self) {
        self.stack.push(self.iter.clone());
    }

    fn pop(&mut self) {
        self.iter = self.stack.pop().expect("Error! Stack underflow");
    }

    fn parse_token(&mut self, token_type: lexer::TokenType) -> Result<(), ParserError<'src>> {
        match self.peek() {
            Some(token) if token.token_type == token_type => {
                self.next();
                Ok(())
            }
            Some(token) => Err(ParserError::new(
                format!("expected token {}, found {}", token_type, token.token_type),
                self.token_vec,
                token.span.start,
                token.span.end,
            )),
            None => Err(ParserError::new_end(
                format!("expected token {}, found end of input", token_type),
                self.token_vec,
            )),
        }
    }

    fn parse_ident(&mut self) -> Result<&'src str, ParserError<'src>> {
        match self.peek() {
            Some(&lexer::Token { token_type: lexer::TokenType::Ident(name), .. }) => {
                self.next();
                Ok(name)
            }
            Some(token) => Err(ParserError::new(
                format!("expected identifier, found {}", token.token_type),
                self.token_vec,
                token.span.start,
                token.span.end,
            )),
            None => Err(ParserError::new_end(
                "expected identifier, found end of input".into(),
                self.token_vec,
            )),
        }
    }

    fn parse_number(&mut self) -> Result<u64, ParserError<'src>> {
        match self.peek() {
            Some(&lexer::Token { token_type: lexer::TokenType::Num(num), .. }) => {
                self.next();
                Ok(num)
            }
            Some(token) => Err(ParserError::new(
                format!("expected number, found {}", token.token_type),
                self.token_vec,
                token.span.start,
                token.span.end,
            )),
            None => Err(ParserError::new_end(
                "expected number, found end of input".into(),
                self.token_vec,
            )),
        }
    }

    fn parse_line(&mut self) -> Result<Box<ast::Ast<'src>>, ParserError<'src>> {
        let err1 = match self.parse_assignment() {
            Ok(ast) => return Ok(ast),
            Err(err) => err,
        };
        let err2 = match self.parse_expression() {
            Ok(ast) => return Ok(ast),
            Err(err) => err,
        };
        unimplemented!()
    }

    fn parse_assignment(&mut self) -> ParserResult<'src> {
        let name = self.parse_ident()?;
        self.parse_token(lexer::TokenType::Assign)?;
        let expr = self.parse_expression()?;
        Ok(Box::new(ast::Ast::Assign(name, expr)))
    }

    fn parse_expression(&mut self) -> ParserResult<'src> {
        unimplemented!()
    }
}


