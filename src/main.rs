mod parser;
mod compiler;
pub mod ast;
use std::{fs::{self, File}, env::args};

fn main() {
    let mut args = args();
    args.next().unwrap();
    let code = fs::read_to_string(args.next().unwrap()).unwrap();

    let ast = parser::ProgramParser::new().parse(&code).unwrap();
    let out = compiler::compile(&ast);
    fs::write("a.out.c", out.as_bytes());
    println!("{}", out);
}
