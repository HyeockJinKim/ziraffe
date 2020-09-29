use crate::error::{CompileError, CompileErrorType};
use crate::symbol_table::Symbol;
use num_bigint::BigUint;
use ziraffe_parser::ast::Operator;
use ziraffe_parser::location::Location;

type InstructionResult<T> = Result<T, CompileError>;

#[derive(Debug)]
pub enum Instruction {
    // Bypass
    Add {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Sub {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Mul {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Div {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Pow {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Assign {
        dst: Box<Symbol>,
        src: Box<Symbol>,
    },
    And {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Or {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Lt {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Le {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Gt {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Ge {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Eq {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    NotEq {
        dst: Box<Symbol>,
        left: Box<Symbol>,
        right: Box<Symbol>,
    },
    Init {
        name: Box<Symbol>,
    },
    InitAssign {
        name: Box<Symbol>,
        src: Box<Symbol>,
    },

    If {
        cond: Box<Symbol>,
        block: Box<Block>,
    },
    For {
        iter: Box<Symbol>,
        start: BigUint,
        end: BigUint,
        block: Box<Block>,
    },
    Else {
        cond: Box<Symbol>,
        block: Box<Block>,
    },
    Call {
        dst: Box<Symbol>,
        func: Box<Symbol>,
        args: Vec<Symbol>,
    },
}

impl Instruction {
    pub fn get_instruction_from_bin_op(
        op: Operator,
        dst: Symbol,
        a: Symbol,
        b: Symbol,
        loc: Location,
    ) -> InstructionResult<Instruction> {
        match op {
            Operator::Add => Ok(Instruction::Add {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Sub => Ok(Instruction::Sub {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Mul => Ok(Instruction::Mul {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Div => Ok(Instruction::Div {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Pow => Ok(Instruction::Pow {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::And => Ok(Instruction::And {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Or => Ok(Instruction::Or {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Lt => Ok(Instruction::Lt {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Le => Ok(Instruction::Le {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Gt => Ok(Instruction::Gt {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Ge => Ok(Instruction::Ge {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::Eq => Ok(Instruction::Eq {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            Operator::NotEq => Ok(Instruction::NotEq {
                dst: Box::new(dst),
                left: Box::new(a),
                right: Box::new(b),
            }),
            _ => Err(CompileError {
                error: CompileErrorType::SyntaxError(String::from("Unreachable")),
                location: loc,
            }),
        }
    }
}

#[derive(Debug, Default)]
pub struct Block {
    codes: Vec<Instruction>,
}

impl Block {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.codes.push(instruction);
    }
}
