use super::lexer::Token;
#[derive(Debug)]
pub enum ASTNode {
    PrintStatement(String),
    Error(String),
    Warn(String),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    ExpectedString,
    ExpectedSemicolon,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn consume(&mut self) {
        self.position += 1;
    }

    fn parse_command<F>(&mut self, constructor: F) -> Result<ASTNode, ParseError> where F: FnOnce(String) -> ASTNode {
        self.consume();

        if !matches!(self.peek(),Token::Arrow) {
            return Err(ParseError::UnexpectedToken);
        }
        self.consume();
        if let Token::StringLit(s) = self.peek() {
            let value = s.clone();
            self.consume();

            match self.peek() {
                Token::Semicolon => self.consume(),
                _ => return Err(ParseError::ExpectedSemicolon),
            }

            Ok(constructor(value))
        } else {
            Err(ParseError::ExpectedString)
        }
    }



    pub fn parse(&mut self) -> Result<Vec<ASTNode>, ParseError> { //
        let mut nodes: Vec<ASTNode> = Vec::new();

        while !matches!(self.peek(), Token::EOF) {
            // Look for pattern

            if let Token::Identifier(ident) = self.peek() {
                match ident.as_str() {
                    "print" => {
                        let node = self.parse_command(ASTNode::PrintStatement)?;
                        nodes.push(node);
                    }
                    "error" => {
                        let node = self.parse_command(ASTNode::Error)?;
                        nodes.push(node);
                    }
                    "warn" => {
                        let node = self.parse_command(ASTNode::Warn)?;
                        nodes.push(node);
                    }
                    _ => {}
                }
            }
        }
        Ok(nodes)
    }
}


