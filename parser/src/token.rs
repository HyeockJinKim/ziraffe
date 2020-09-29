use num_bigint::BigUint;

/// Ziraffe source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    // Operator
    // Arithmetic Operator
    Plus,
    Minus,
    Mul,
    Div,
    // Power Operator
    Pow,
    // Assign operator
    Assign,
    // Logical Operator
    And,
    Or,
    // Comparison Operator
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    NotEq,

    // Type
    URL,
    JSON,
    // Static size
    Uint,
    Bool,
    String,
    Address,

    // Keyword
    Function,
    Contract,
    If,
    Else,
    For,
    In,
    Returns,
    // Mark
    LPar,
    RPar,
    LBrace,
    RBrace,
    Semi,
    Comma,
    Dot,
    DotDot,
    // variable
    Num { number: BigUint },
    Literal { literal: String },
    Identifier { name: String },
    EOF,
}
