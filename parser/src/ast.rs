use num_bigint::BigUint;

use crate::location::Location;

// https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq)]
pub enum Program {
    GlobalStatements(Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type Statement = Located<StatementType>;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    // Global Statement
    FunctionStatement {
        function_name: Box<Expression>,
        parameters: Box<Expression>,
        expr: Box<Expression>,
        returns: Option<Type>,
    },
    ContractStatement {
        contract_name: Box<Expression>,
        members: Box<Statement>,
    },
    InitializerStatement {
        variable_type: Type,
        variable: Box<Expression>,
        default: Option<Box<Expression>>,
    },
    // Local Statement
    MemberStatement {
        statements: Vec<Statement>,
    },
    Expression {
        expression: Box<Expression>,
    },
}

pub type Expression = Located<ExpressionType>;

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    CompoundExpression {
        statements: Vec<Statement>,
        return_value: Option<Box<Expression>>,
    },
    AssignExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    FunctionCallExpression {
        function_name: Box<Expression>,
        arguments: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        if_expr: Box<Expression>,
        else_expr: Option<Box<Expression>>,
    },
    ForEachExpression {
        iterator: Box<Expression>,
        vector: Box<Expression>,
        for_expr: Box<Expression>,
    },
    Parameters {
        parameters: Vec<Statement>,
    },
    Arguments {
        arguments: Vec<Expression>,
    },
    Range {
        start: BigUint,
        end: BigUint,
    },
    Literal {
        value: String,
    },
    Number {
        value: BigUint,
    },
    Identifier {
        value: String,
    },
}

impl ExpressionType {
    pub fn identifier_name(&self) -> Option<String> {
        if let ExpressionType::Identifier { value } = self {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic Operator
    Add,
    Sub,
    Mul,
    Div,
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
}

#[derive(Debug, PartialEq)]
pub enum Type {
    // type
    URL,
    JSON,
    // Static size
    Uint,
    Bool,
    String,
    Address,
}
