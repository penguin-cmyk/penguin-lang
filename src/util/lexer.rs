pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    TypeNumber,
    TypeString,
    TypeBoolean,

    StringLiteral(String),
    Identifier(String),
    Comment(String),
    DoubleEquals,
    DoubleColon,
    FileSystem,
    VariableDec,
    Number(f64),
    Boolean(bool),
    Panic,
    Comma,
    Semicolon,
    LCurlyBrace,
    RCurlyBrace,
    LBracket,
    QuestionMark,
    RBracket,
    NotEqual,
    Divide,
    Elseif,
    LBrace,
    RBrace,
    Equals,
    Colon,
    Arrow,
    Minus,
    Print,
    Plus,
    Mult,
    Else,
    Not,
    Dot,
    Std,
    Eof,
    Use,
    If,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }
    fn peek(&self) -> Option<char> { self.input.get(self.position).copied() }
    fn predict(&mut self,character: Option<char> ,if_char: Token, if_not_char: Token,tokens: &mut Vec<Token>) {
        self.position += 1;
        if self.peek() == character {
            tokens.push(if_char);
        } else {
            tokens.push(if_not_char);
        }
        self.position += 1;
    }
    fn collect(&mut self, exceptions: Vec<char> ) -> String {
        self.position += 1;
        let start = self.position;
        while let Some(character) = self.peek() {

            if exceptions.contains(&character) { break; }
            self.position += 1;
        }
        let output: String = self.input[start..self.position].iter().collect();
        output
    }
    
    fn push(&mut self, token: Token, tokens: &mut Vec<Token>) {
        tokens.push(token);
        self.position += 1;
    }
    
    pub fn lex(&mut self) -> Vec<Token> {

        let mut tokens: Vec<Token> = Vec::new();

        while self.position < self.input.len() {
            let character: char = self.input[self.position];
            
            match character {
                'a'..='z' | 'A'..='Z' => {
                    let start = self.position;
                    while let Some('a'..='z' | 'A'..='Z') = self.peek() {
                        self.position += 1;
                    }
                    let identifier: String = self.input[start..self.position].iter().collect();
                    match identifier.as_str() {
                        "let" => tokens.push(Token::VariableDec),
                        "fs" => tokens.push(Token::FileSystem),
                        "std" => tokens.push(Token::Std),
                        "if" => tokens.push(Token::If), 
                        "use" => tokens.push(Token::Use),
                        "print" => tokens.push(Token::Print),
                        "panic" => tokens.push(Token::Panic),
                        "elseif" => tokens.push(Token::Elseif),
                        "else" => tokens.push(Token::Else),
                        "number" => tokens.push(Token::TypeNumber),
                        "bool" => tokens.push(Token::TypeBoolean),
                        "string" => tokens.push(Token::TypeString),
                        "true" => tokens.push(Token::Boolean(true)),
                        "false" => tokens.push(Token::Boolean(false)),
                        _ =>  tokens.push(Token::Identifier(identifier)),
                    };
                }
                '0'..='9' => {
                    let start = self.position;
                    while let Some('0'..='9') = self.peek() { self.position += 1; }
                    if let Some('.') = self.peek() { 
                        self.position += 1; 
                        while let Some('0'..='9') = self.peek() { 
                            self.position += 1; 
                        }
                    }

                    let number: String = self.input[start..self.position].iter().collect();
                    match number.parse::<f64>() {
                        Ok(num) => tokens.push(Token::Number(num)),
                        Err(_) => panic!("Error occured when trying to turn string into number"),
                    }
                }
                '/' => {
                    self.position += 1;
                    match self.peek() {
                        Some('/') => {
                           let comment: String = self.collect(vec!['\n']);
                           tokens.push(Token::Comment(comment));
                        },
                        _ => tokens.push(Token::Divide),
                    }
                },
                '"' => {
                    let literal: String = self.collect(vec!['"']);
                    tokens.push(Token::StringLiteral(literal));
                    self.position += 1;
                }

                ';' => self.push(Token::Semicolon, &mut tokens),
                '[' => self.push(Token::LBracket,&mut tokens),
                ']' => self.push(Token::RBracket, &mut tokens),
                '{' => self.push(Token::LCurlyBrace,&mut tokens),
                '}' => self.push(Token::RCurlyBrace, &mut tokens),
                '(' => self.push(Token::LBrace, &mut tokens),
                ')' => self.push(Token::RBrace, &mut tokens),
                '+' => self.push(Token::Plus, &mut tokens),
                '*' => self.push(Token::Mult, &mut tokens),
                '.' => self.push(Token::Dot, &mut tokens),
                '?' => self.push(Token::QuestionMark,&mut tokens),
                ',' => self.push(Token::Comma,&mut tokens),

                '~' => self.predict(Some('='), Token::NotEqual,Token::Not,&mut tokens),
                ':' => self.predict(Some(':'), Token::DoubleColon, Token::Colon, &mut tokens),
                '=' => self.predict(Some('='),Token::DoubleEquals,Token::Equals,&mut tokens),
                '-' => self.predict(Some('>'), Token::Arrow, Token::Minus , &mut tokens),

                ' ' | '\n' | '\t' => self.position += 1,
                _ => panic!("Unexpected character: {character}"),
            }
        }
        tokens.push(Token::Eof);
        tokens 
    }

}

