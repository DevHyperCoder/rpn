use crate::token::*;

pub fn parse(input: String) -> Vec<Token> {
    let mut tokens = vec![];

    for word in input.trim().split(' ') {
        if word.is_empty() {
            continue;
        }

        if let Ok(num) = word.parse::<f64>() {
            tokens.push(Token::Number(num))
        } else if let Some(numeric_op) = parse_numeric_op(word) {
            tokens.push(numeric_op)
        } else {
            tokens.push(Token::Word(word.to_string()))
        }
    }

    tokens
}

pub fn parse_numeric_op(input: &str) -> Option<Token> {
    Some(match input {
        "+" => Token::NumericOp(NumericOp::Plus),
        "-" => Token::NumericOp(NumericOp::Minus),
        "*" => Token::NumericOp(NumericOp::Mul),
        "/" => Token::NumericOp(NumericOp::Div),
        "^" => Token::NumericOp(NumericOp::Exponent),
        "=" => Token::Op(Op::Assign),
        "@" => Token::Op(Op::AssertOnStack),

        "cls" => Token::Op(Op::Clear),
        "print" => Token::Op(Op::Print),
        "print-stack" => Token::Op(Op::PrintStack),
        "print-var" => Token::Op(Op::PrintVariables),
        "dup" => Token::Op(Op::Duplicate),
        "drp" => Token::Op(Op::Drop),
        "swp" => Token::Op(Op::Swap),
        "inc" => Token::Op(Op::Include),
        "def" => Token::Op(Op::Define),
        "end" => Token::Op(Op::EndDefine),

        _ => return None,
    })
}
