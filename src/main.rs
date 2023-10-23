mod gen;
mod parser;
mod token;

use std::env;

use gen::gen;
use parser::Parser;
use token::tokenize;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("invalid arguments!");
    }

    args.next().unwrap();
    let tokens = tokenize(args.next().unwrap());
    let mut parser = Parser::new(tokens);
    let node = parser.parse();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}
