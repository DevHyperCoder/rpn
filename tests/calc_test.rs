use std::vec;

use rpn::{
    calculator::Calculator,
    stack_type::StackType,
    token::{NumericOp, Op, Token},
};

#[test]
fn it_tests_basics() {
    let mut calc = Calculator::new();

    calc.process_tokens(vec![
        Token::Number(1.),
        Token::Number(1.),
        Token::NumericOp(NumericOp::Plus),
    ]);

    assert_eq!(calc.get_last(), Some(&StackType::Number(2.)));

    calc.process_tokens(vec![Token::Number(2.), Token::NumericOp(NumericOp::Minus)]);

    assert_eq!(calc.get_last(), Some(&StackType::Number(0.)));
}

#[test]
fn it_tests_variables() {
    let mut calc = Calculator::new();

    calc.process_tokens(vec![
        Token::Number(10.),
        Token::Word("test".into()),
        Token::Op(Op::Assign),
    ]);

    assert_eq!(calc.get_variable_value("test"), Some(10.));

    // Assigning to a variable doesn't put it on stack.
    assert_eq!(calc.get_last(), None);

    calc.process_tokens(vec![Token::Word("test".into())]);

    assert_eq!(
        calc.get_last(),
        Some(&StackType::Variable("test".to_string()))
    );

    calc.process_tokens(vec![Token::Number(0.), Token::NumericOp(NumericOp::Plus)]);

    assert_eq!(calc.get_last(), Some(&StackType::Number(10.)));
}
