#[derive(Debug,Clone)]
pub enum Token {
    Identifier(String),
    Arrow, // ->
    StringLit(String), // "text"
    EOF, // end of line
    Semicolon,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    // Helper to peek at next char
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let c = self.input[self.position];

            match c {
                ' ' | '\t' | '\n' => self.position += 1, // skip white spaces
                'a'..='z' | 'A'..='Z' => {
                    let start = self.position;
                    while let Some('a'..='z' | 'A'..='Z') = self.peek() {
                        self.position += 1;
                    }
                    let ident: String = self.input[start..self.position].iter().collect();
                    tokens.push(Token::Identifier(ident));
                }
                '"' => {
                    self.position += 1; // skip opening quote
                    let start = self.position;

                    while let Some(ch) = self.peek() {
                        if ch == '"' { break;}
                        self.position += 1;
                    }
                    let string: String = self.input[start..self.position].iter().collect();
                    tokens.push(Token::StringLit(string));
                    self.position += 1;
                }

                '-' => {
                    self.position += 1;
                    if let Some('>') = self.peek() {
                        self.position += 1;
                        tokens.push(Token::Arrow);
                    } else {
                        panic!("Unexpected character after '-'");
                    }
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.position += 1;
                }
                _ => panic!("Unexpected character {}", c),
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}