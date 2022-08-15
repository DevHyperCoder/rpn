use rpn::{
    parser::parse,
    token::{NumericOp, Op, Token},
};

#[test]
fn it_tests_basic_parsing() {
    assert_eq!(
        parse("1 2 +".into()),
        vec![
            Token::Number(1.),
            Token::Number(2.),
            Token::NumericOp(NumericOp::Plus)
        ]
    );

    assert_eq!(
        parse("1.12 -0.12 /".into()),
        vec![
            Token::Number(1.12),
            Token::Number(-0.12),
            Token::NumericOp(NumericOp::Div)
        ]
    );

    assert_eq!(
        parse("3.14 pi =".into()),
        vec![
            Token::Number(3.14),
            Token::Word("pi".into()),
            Token::Op(Op::Assign)
        ]
    )
}
