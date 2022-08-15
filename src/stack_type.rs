use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum StackType {
    Variable(String),
    Number(f64),
}

impl fmt::Display for StackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackType::Number(n) => write!(f, "{}", n),
            StackType::Variable(var) => write!(f, "{}", var),
        }
    }
}
