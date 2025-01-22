mod util;
use std::fs;
use util::{Tokenizer,Parser,Interpreter};

/*
   Todo:
       - Add variables ( numbers, arrays, maybe tuples, strings )
       - Add if | else | else if statements
       - Also add keywords
       - Add operators
       - Add loops
       - Add standard library ( File system, ...)
       - Add types (f32, char,...)
       - Add return
       - Add comments
       - (later) -> add back support for rust crates
 */

fn main() {
    let code = match fs::read_to_string("main.penguin") {
        Ok(content) => content,
        _ => {eprintln!("main.penguin does not exist");return ();}
    };

    let tokens = Tokenizer::new(&code.as_str()).tokenize();
    let ast = Parser::new(tokens).parse().unwrap();

    Interpreter::run(ast);
}