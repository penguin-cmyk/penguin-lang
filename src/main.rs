mod util;
use std::fs;
use util::Lexer;

fn main() {
    let code: String = match fs::read_to_string("src/main.p") {
        Ok(cod) => cod,
        _ => panic!("File doesnt exist"),
    };
    let tokens = Lexer::new(&code.as_str()).lex();
    
    for token in &tokens {
        println!("{:?}",token);
    }
}
