#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(f64),
    NumericOp(NumericOp),
    Op(Op),
    Word(String),
}

// Base operations
#[derive(PartialEq, Debug, Clone)]
pub enum NumericOp {
    Plus,
    Minus,
    Mul,
    Div,
    Exponent,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Op {
    Clear,
    Assign,
    Swap,
    Drop,
    Duplicate,
    Print,
    PrintStack,
    PrintVariables,
    Include,
    Define,
    EndDefine,
    AssertOnStack,
}
