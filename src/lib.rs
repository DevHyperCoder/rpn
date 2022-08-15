pub mod calculator;
pub mod parser;
pub mod stack_type;
pub mod token;

use std::io::{stdin, stdout, Write};

use crate::{
    calculator::Calculator,
    parser::parse,
    token::{Op, Token},
};

const PROMPT: &str = ">> ";

pub fn run() {
    let mut calc = Calculator::new();
    let mut input = String::new();

    loop {
        print!("{}", PROMPT);
        stdout().flush().expect("Could not flush stdout.");
        stdin()
            .read_line(&mut input)
            .expect("Could not read from stdin.");

        let inp_cl = input.clone();
        calc.process_tokens(parse(inp_cl));

        calc.process_tokens(vec![Token::Op(Op::Print)]);
        input.clear();
    }
}
