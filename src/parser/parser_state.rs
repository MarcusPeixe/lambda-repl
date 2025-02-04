use super::*;

#[derive(Debug, Clone, Copy)]
struct Flags {
    ignore_newline: bool,
}

pub struct ParserState<'src> {
    token_vec: &'src lexer::TokenVec<'src>,
    pub errors: Vec<ParserError<'src>>,
    iter: lexer::TokenIter<'src>,
    stack: Vec<lexer::TokenIter<'src>>,
}

impl<'src> ParserState<'src> {
    pub fn new(token_vec: &'src lexer::TokenVec<'src>) -> Self {
        let iter = token_vec.iter();
        Self {
            token_vec,
            errors: Vec::new(),
            iter,
            stack: Vec::new(),
        }
    }

    pub fn parse_line(&mut self) -> ParserResult<'src> {
        self.push();
        let err1 = match self.parse_assignment(Flags { ignore_newline: false }) {
            Ok(ast) => {
                self.discard();
                return Ok(ast)
            }
            Err(err) => err,
        };
        self.pop();
        let err2 = match self.parse_abstraction(Flags { ignore_newline: false }) {
            Ok(ast) => return Ok(ast),
            Err(err) => err,
        };
        Err(ParserError::get_longer_of(err1, err2).to_singleton(self.token_vec))
    }

    pub fn parse_file(&mut self) -> ParserResult<'src> {
        let mut asts = Vec::new();
        loop {
            self.skip_newlines(Flags { ignore_newline: true });
            match self.parse_assignment(Flags { ignore_newline: false }) {
                Ok(ast) => asts.push(*ast),
                Err(err) => {
                    self.errors.push(err);
                    self.sync_to_newline();
                    if self.peek().is_none() {
                        break;
                    }
                }
            }
        }
        Ok(Box::new(ast::Ast::Source(asts)))
    }

    fn peek(&mut self) -> Option<&'src lexer::Token<'src>> {
        loop {
            match self.iter.peek().copied() {
                Some(lexer::Token { token_type: lexer::TokenType::Comment(_), .. }) => {
                    self.iter.next();
                }
                token => return token,
            }
        }
    }

    fn next(&mut self) -> Option<&'src lexer::Token<'src>> {
        loop {
            match self.iter.next() {
                Some(lexer::Token { token_type: lexer::TokenType::Comment(_), .. }) => {}
                token => return token,
            }
        }
    }

    fn push(&mut self) {
        self.stack.push(self.iter.clone());
    }

    fn discard(&mut self) {
        self.stack.pop();
    }

    fn pop(&mut self) {
        self.iter = self.stack.pop().expect("Error! Stack underflow");
    }

    fn parse_token(&mut self, token_type: lexer::TokenType, flags: Flags) -> Result<(), ParserError<'src>> {
        self.skip_newlines(flags);
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

    fn parse_ident(&mut self, flags: Flags) -> Result<&'src str, ParserError<'src>> {
        self.skip_newlines(flags);
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

    fn parse_number(&mut self, flags: Flags) -> Result<u64, ParserError<'src>> {
        self.skip_newlines(flags);
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

    fn skip_newlines(&mut self, flags: Flags) {
        if !flags.ignore_newline {
            return;
        }
        while let Some(token) = self.peek() {
            if token.token_type == lexer::TokenType::Eol {
                self.next();
            } else {
                break;
            }
        }
    }

    fn sync_to_newline(&mut self) {
        while let Some(token) = self.next() {
            if token.token_type == lexer::TokenType::Eol {
                break;
            }
        }
    }

    fn parse_assignment(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let name = self.parse_ident(flags)?;
        self.parse_token(lexer::TokenType::Assign, flags)?;
        let expr = self.parse_abstraction(flags)?;
        self.parse_token(lexer::TokenType::Eol, flags)?;
        Ok(Box::new(ast::Ast::Assign(name, expr)))
    }

    fn parse_abstraction(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        if self.parse_token(lexer::TokenType::Lambda, flags).is_ok() {
            let arg = self.parse_ident(flags)?;
            self.parse_token(lexer::TokenType::Dot, flags)?;
            let body = self.parse_abstraction(flags)?;
            Ok(Box::new(ast::Ast::Abs(arg, body)))
        } else {
            self.parse_comparison(flags)
        }
    }

    fn parse_comparison(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let mut expr = self.parse_disjunction(flags)?;
        loop {
            self.skip_newlines(flags);
            match self.peek() {
                Some(token) if token.token_type == lexer::TokenType::Eq => {
                    self.next();
                    expr = Box::new(ast::Ast::Eq(expr, self.parse_disjunction(flags)?));
                }
                Some(token) if token.token_type == lexer::TokenType::Neq => {
                    self.next();
                    expr = Box::new(ast::Ast::Neq(expr, self.parse_disjunction(flags)?));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_disjunction(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let mut expr = self.parse_conjunction(flags)?;
        loop {
            self.skip_newlines(flags);
            match self.peek() {
                Some(token) if token.token_type == lexer::TokenType::Or => {
                    self.next();
                    expr = Box::new(ast::Ast::Or(expr, self.parse_conjunction(flags)?));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_conjunction(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let mut expr = self.parse_sum(flags)?;
        loop {
            self.skip_newlines(flags);
            match self.peek() {
                Some(token) if token.token_type == lexer::TokenType::And => {
                    self.next();
                    expr = Box::new(ast::Ast::And(expr, self.parse_sum(flags)?));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_sum(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let mut expr = self.parse_product(flags)?;
        loop {
            self.skip_newlines(flags);
            match self.peek() {
                Some(token) if token.token_type == lexer::TokenType::Add => {
                    self.next();
                    expr = Box::new(ast::Ast::Add(expr, self.parse_product(flags)?));
                }
                Some(token) if token.token_type == lexer::TokenType::Sub => {
                    self.next();
                    expr = Box::new(ast::Ast::Sub(expr, self.parse_product(flags)?));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_product(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let mut expr = self.parse_application(flags)?;
        loop {
            self.skip_newlines(flags);
            match self.peek() {
                Some(token) if token.token_type == lexer::TokenType::Mul => {
                    self.next();
                    expr = Box::new(ast::Ast::Mul(expr, self.parse_application(flags)?));
                }
                Some(token) if token.token_type == lexer::TokenType::Div => {
                    self.next();
                    expr = Box::new(ast::Ast::Div(expr, self.parse_application(flags)?));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_application(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        let mut expr = self.parse_unary(flags)?;
        while let Ok(arg) = self.parse_abstraction(flags) {
            expr = Box::new(ast::Ast::App(expr, arg));
        }
        Ok(expr)
    }

    fn parse_unary(&mut self, flags: Flags) -> Result<ast::Node<'src>, ParserError<'src>> {
        self.skip_newlines(flags);
        match self.peek() {
            Some(token) if token.token_type == lexer::TokenType::LPar => {
                self.next();
                let expr = self.parse_abstraction(Flags { ignore_newline: true })?;
                self.parse_token(lexer::TokenType::RPar, flags)?;
                Ok(expr)
            }
            Some(token) if token.token_type == lexer::TokenType::Not => {
                self.next();
                Ok(Box::new(ast::Ast::Not(self.parse_unary(flags)?)))
            }
            Some(lexer::Token { token_type: lexer::TokenType::Ident(_), .. }) => {
                Ok(Box::new(ast::Ast::Var(self.parse_ident(flags)?)))
            }
            Some(lexer::Token { token_type: lexer::TokenType::Num(_), .. }) => {
                Ok(Box::new(ast::Ast::Num(self.parse_number(flags)?)))
            }
            Some(token) => Err(ParserError::new(
                format!("expected expression, found {}", token.token_type),
                self.token_vec,
                token.span.start,
                token.span.end,
            )),
            None => Err(ParserError::new_end(
                "expected expression, found end of input".into(),
                self.token_vec,
            )),
        }
    }
}


