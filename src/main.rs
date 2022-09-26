pub mod lexer;
pub mod parser;
pub mod repl;

use std::io;
use std::io::Write;
use lexer::Lexer;
use parser::Parser;
use repl::Repl;

fn main() {
    let mut parser = Parser::new();
    let mut repl = Repl::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
        stdout.write(b">> ");
        stdout.flush();
        stdin.read_line(&mut line);

        if line == "quit".to_string() {
            break;
        }

        let tokens = Lexer::lex(line.clone());
        let ast = parser.parse(tokens);
        println!("ast -> {ast:?}");
        let res = repl.interpret(ast.unwrap());

        println!("-> {res:?}");
    }
}
