use crate::compiler::{lexer::tokenize, parser::parse};

pub(crate) mod lexer;
pub(crate) mod parser;

pub struct ByteCode {}

pub(crate) fn compile(source: &str) -> ByteCode {
    let tokens = tokenize(source).unwrap();
    parse(&tokens).unwrap();

    todo!()
}
