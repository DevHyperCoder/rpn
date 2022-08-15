use crate::parser::parse;
use crate::stack_type::StackType;
use crate::token::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Calculator {
    var_to_val: HashMap<String, f64>,
    stack: Vec<StackType>,

    functions: HashMap<String, Vec<Token>>,

    is_defining_func: Option<String>,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator::new()
    }
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            stack: vec![],
            functions: HashMap::new(),
            is_defining_func: None,
            var_to_val: HashMap::new(),
        }
    }

    pub fn get_last(&self) -> Option<&StackType> {
        self.stack.last()
    }

    pub fn get_variable_value(&self, v: &str) -> Option<f64> {
        self.var_to_val.get(v).copied()
    }

    fn do_operation(&mut self, f: fn(f64, f64) -> f64) -> Option<()> {
        let b = match self.stack.pop()? {
            StackType::Number(e) => e,
            StackType::Variable(v) => *self.var_to_val.get(&v)?,
        };
        let a = match self.stack.pop()? {
            StackType::Number(e) => e,
            StackType::Variable(v) => *self.var_to_val.get(&v)?,
        };

        self.stack.push(StackType::Number(f(a, b)));
        Some(())
    }

    pub fn process_tokens(&mut self, tokens: Vec<Token>) {
        for token in tokens {
            if let Token::Op(Op::EndDefine) = token {
                self.is_defining_func = None;
                continue;
            }

            if let Some(fname) = &self.is_defining_func {
                self.functions
                    .entry(fname.to_string())
                    .and_modify(|tks| tks.push(token));
                continue;
            }

            match token {
                Token::Word(w) => {
                    match self.functions.get(&w).cloned() {
                        Some(f) => self.process_tokens(f.to_vec()),
                        None => self.stack.push(StackType::Variable(w.to_string())),
                    };
                }
                Token::Number(n) => self.stack.push(StackType::Number(n)),
                Token::Op(op) => match op {
                    Op::AssertOnStack => {
                        let n = match self.stack.pop() {
                            None => {
                                eprintln!("not enough elem");
                                return;
                            }
                            Some(s) => match s {
                                StackType::Number(n) => n,
                                StackType::Variable(v) => {
                                    if let Some(e) = self.get_variable_value(&v) {
                                        e
                                    } else {
                                        eprintln!("not find var val");
                                        return;
                                    }
                                }
                            },
                        };

                        if self.stack.len() < n as usize {
                            eprintln!("Not enough elements on stack.");
                            return;
                        }
                    }
                    Op::Define => {
                        if self.is_defining_func.is_some() {
                            eprintln!("Can not nest functions inside each other.");
                            return;
                        }

                        let fun_name = match self.stack.pop() {
                            None => {
                                eprintln!("function name not provided");
                                return;
                            }
                            Some(s) => match s {
                                StackType::Variable(v) => v,
                                _ => {
                                    eprintln!("Expected name, got number");
                                    return;
                                }
                            },
                        };

                        self.is_defining_func = Some(fun_name.clone());
                        self.functions.insert(fun_name, vec![]);
                    }
                    Op::EndDefine => unreachable!(),
                    Op::Clear => {
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    Op::Include => {
                        let file_name = match self.stack.pop() {
                            None => {
                                eprintln!("File name not provided.");
                                return;
                            }
                            Some(f) => match f {
                                StackType::Variable(v) => v,
                                _ => {
                                    eprintln!("File name not provided.");
                                    return;
                                }
                            },
                        };

                        let mut f = match File::open(&file_name) {
                            Err(e) => {
                                eprintln!("Unable to open file: {}", e);
                                return;
                            }
                            Ok(e) => e,
                        };

                        let mut file_content = String::new();
                        if let Err(e) = f.read_to_string(&mut file_content) {
                            eprintln!("Could not read {}: {}", file_name, e);
                            return;
                        }

                        for line in file_content.split('\n') {
                            self.process_tokens(parse(line.to_string()))
                        }
                    }
                    Op::Print => {
                        if let Some(a) = self.get_last() {
                            match a {
                                StackType::Number(n) => println!("{}", n),
                                StackType::Variable(var) => {
                                    let val = match self.var_to_val.get(var) {
                                        None => "NULL".to_string(),
                                        Some(s) => s.to_string(),
                                    };

                                    println!("{} => {}", var, val);
                                }
                            }
                        }
                    }
                    Op::PrintStack => {
                        for s in self.stack.iter() {
                            match s {
                                StackType::Number(n) => println!("{}", n),
                                StackType::Variable(var) => {
                                    let val = match self.var_to_val.get(var) {
                                        None => "NULL".to_string(),
                                        Some(s) => s.to_string(),
                                    };

                                    println!("{} => {}", var, val);
                                }
                            }
                        }
                    }
                    Op::PrintVariables => {
                        for (var, val) in self.var_to_val.iter() {
                            println!("{} => {}", var, val);
                        }
                    }
                    Op::Duplicate => match self.stack.pop() {
                        Some(l) => {
                            self.stack.push(l.clone());
                            self.stack.push(l.clone());
                        }
                        None => {
                            eprintln!("Expected a element on stack.");
                            return;
                        }
                    },
                    Op::Drop => {
                        self.stack.pop();
                    }
                    Op::Assign => {
                        let var_name = match self.stack.pop() {
                            None => {
                                eprintln!("Could not find variable name.");
                                return;
                            }
                            Some(s) => match s {
                                StackType::Variable(var) => var,
                                _ => {
                                    eprintln!("Expected variable on top of the stack.");
                                    return;
                                }
                            },
                        };

                        let val = match self.stack.pop() {
                            None => {
                                eprintln!("Could not find variable value.");
                                return;
                            }
                            Some(v) => match v {
                                StackType::Number(n) => n,
                                StackType::Variable(v) => match self.var_to_val.get(&v) {
                                    Some(val) => *val,
                                    None => {
                                        eprintln!("Variable not found.");
                                        return;
                                    }
                                },
                            },
                        };

                        self.var_to_val.insert(var_name, val);
                    }
                    Op::Swap => {
                        if self.stack.len() < 2 {
                            eprintln!("Not enough elements to swap.");
                            return;
                        }

                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();

                        self.stack.push(b);
                        self.stack.push(a);
                    }
                },
                Token::NumericOp(o) => {
                    type OpFun = fn(f64, f64) -> f64;
                    let f: OpFun = match o {
                        NumericOp::Plus => |a, b| a + b,
                        NumericOp::Minus => |a, b| a - b,
                        NumericOp::Mul => |a, b| a * b,
                        NumericOp::Div => |a, b| a / b,
                        NumericOp::Exponent => |a, b| a.powf(b),
                    };

                    if self.do_operation(f).is_none() {
                        eprintln!("Insuff args or variable not found")
                    }
                }
            }
        }
    }
}
