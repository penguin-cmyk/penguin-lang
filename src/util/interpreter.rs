use super::parser::ASTNode;
pub struct Interpreter;

use console::style;

impl Interpreter {
    pub fn run(nodes: Vec<ASTNode>) {
        for node in nodes {
            match node {
                ASTNode::PrintStatement(s) => println!("{}", s),
                ASTNode::Error(e) => println!("{}",style(e).red()),
                ASTNode::Warn(w) => println!("{}",style(w).yellow()),
            }
        }
    }
}