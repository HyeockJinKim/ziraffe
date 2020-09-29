use crate::error::{CompileError, CompileErrorType};
use crate::instruction::{Block, Instruction};
use crate::symbol_table::{Context, Contract, Function, Symbol, Type};
use indexmap::map::IndexMap;
use num_bigint::BigUint;
use ziraffe_parser::ast;
use ziraffe_parser::ast::{ExpressionType, Program, StatementType};

pub fn compile_program(program: &ast::Program) -> CompileResult<IndexMap<String, Contract>> {
    let mut compiler = Compiler::new();
    compiler.compile_program(program)?;
    Ok(compiler.contracts)
}

struct Compiler {
    contracts: IndexMap<String, Contract>,
    context: Context,
}

type CompileResult<T> = Result<T, CompileError>;

impl Compiler {
    fn new() -> Self {
        Compiler {
            contracts: Default::default(),
            context: Context::new(),
        }
    }

    fn compile_program(&mut self, ast: &ast::Program) -> CompileResult<()> {
        match ast {
            Program::GlobalStatements(stmts) => {
                for stmt in stmts {
                    self.compile_stmt(stmt)?;
                }
            }
        }
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &ast::Statement) -> CompileResult<Symbol> {
        match &stmt.node {
            ast::StatementType::FunctionStatement {
                function_name,
                parameters,
                expr,
                returns,
            } => {
                self.context.is_member = false;
                let name = self.compile_expr(function_name)?;
                let params = self.compile_param(parameters)?;
                let typ = if let Some(ret) = returns {
                    Type::get_type(ret)
                } else {
                    Type::None
                };
                let block = self.compile_block(expr)?;
                self.add_function(&name.id, params, typ, block);
                self.context.is_member = true;
                Ok(Symbol::temp_symbol(&mut self.context))
            }
            StatementType::ContractStatement {
                contract_name,
                members,
            } => {
                let name = self.compile_expr(contract_name)?;
                self.look_contract(&name.id);
                self.context.is_member = true;
                self.compile_stmt(members)?;
                self.context.is_member = false;
                Ok(Symbol::temp_symbol(&mut self.context))
            }
            StatementType::InitializerStatement {
                variable_type,
                variable,
                default,
            } => {
                let typ = Type::get_type(variable_type);
                let mut symbol = self.compile_expr(variable)?;
                if symbol.typ == Type::Undefined {
                    symbol.typ = typ;
                }
                if self.context.is_member {
                    let contract_name = self.context.current_contract.as_ref().unwrap();
                    let contract = self.contracts.get_mut(contract_name).unwrap();
                    contract
                        .member
                        .insert(symbol.id.to_string(), symbol.clone());
                } else {
                    if let Some(value) = default {
                        // TODO:
                        let src = self.compile_expr(value)?;
                        self.context.add_instruction(Instruction::InitAssign {
                            name: Box::new(symbol.clone()),
                            src: Box::new(src),
                        });
                    } else {
                        self.context.add_instruction(Instruction::Init {
                            name: Box::new(symbol.clone()),
                        });
                    }
                    self.context.add_symbol(symbol.id.as_str(), symbol.clone());
                }
                Ok(symbol)
            }
            StatementType::MemberStatement { statements } => {
                for statement in statements {
                    self.compile_stmt(statement)?;
                }
                Ok(Symbol::temp_symbol(&mut self.context))
            }
            StatementType::Expression { expression } => self.compile_expr(expression),
        }
    }

    fn compile_expr(&mut self, expr: &ast::Expression) -> CompileResult<Symbol> {
        match &expr.node {
            ast::ExpressionType::CompoundExpression {
                statements: _,
                return_value: _,
            } => {
                self.compile_block(expr)?;
                Ok(Symbol::temp_symbol(&mut self.context))
            }
            ExpressionType::AssignExpression {
                left,
                operator: _,
                right,
            } => {
                let a = self.compile_expr(left)?;
                let b = self.compile_expr(right)?;
                self.context.add_instruction(Instruction::Assign {
                    dst: Box::new(a.clone()),
                    src: Box::new(b),
                });
                Ok(a)
            }
            ExpressionType::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let a = self.compile_expr(left)?;
                let b = self.compile_expr(right)?;
                let dst =
                    Symbol::result_symbol(&mut self.context, a.clone(), b.clone(), expr.location)?;
                let res = Instruction::get_instruction_from_bin_op(
                    operator.clone(),
                    dst.clone(),
                    a,
                    b,
                    expr.location,
                )?;
                self.context.add_instruction(res);
                Ok(dst)
            }
            ExpressionType::FunctionCallExpression {
                function_name,
                arguments,
            } => {
                let name = self.compile_expr(function_name)?;
                let args = self.compile_param(arguments)?;
                let res = Symbol::temp_symbol(&mut self.context);
                self.context.add_instruction(Instruction::Call {
                    dst: Box::new(res.clone()),
                    func: Box::new(name),
                    args,
                });
                Ok(res)
            }
            ExpressionType::IfExpression {
                condition,
                if_expr,
                else_expr,
            } => {
                let condition = self.compile_expr(condition)?;
                let if_block = self.compile_block(if_expr)?;
                self.context.add_instruction(Instruction::If {
                    cond: Box::new(condition.clone()),
                    block: Box::new(if_block),
                });
                if let Some(else_expression) = else_expr {
                    let else_block = self.compile_block(else_expression)?;
                    self.context.add_instruction(Instruction::Else {
                        cond: Box::new(condition),
                        block: Box::new(else_block),
                    });
                }
                Ok(Symbol::temp_symbol(&mut self.context))
            }
            ExpressionType::ForEachExpression {
                iterator,
                vector,
                for_expr,
            } => {
                let iter = self.compile_expr(iterator)?;
                let (start, end) = self.compile_range(vector)?;
                let block = self.compile_block(for_expr)?;
                self.context.add_instruction(Instruction::For {
                    iter: Box::new(iter),
                    start,
                    end,
                    block: Box::new(block),
                });
                Ok(Symbol::temp_symbol(&mut self.context))
            }
            ExpressionType::Literal { value } => Ok(Symbol::literal_symbol(value.to_string())),
            ExpressionType::Number { value } => Ok(Symbol {
                id: value.to_string(),
                num: 0,
                typ: Type::Uint,
            }),
            ExpressionType::Identifier { value } => Ok(self.context.get_symbol(value)),
            _ => Err(CompileError {
                error: CompileErrorType::SyntaxError(String::from("Unreachable")),
                location: expr.location,
            }),
        }
    }

    fn compile_param(&mut self, ast: &ast::Expression) -> CompileResult<Vec<Symbol>> {
        match &ast.node {
            ExpressionType::Parameters { parameters } => {
                let mut params = vec![];
                for parameter in parameters {
                    params.push(self.compile_stmt(parameter)?);
                }
                Ok(params)
            }
            ExpressionType::Arguments { arguments } => {
                let mut args = vec![];
                for argument in arguments {
                    args.push(self.compile_expr(argument)?);
                }
                Ok(args)
            }
            _ => Err(CompileError {
                error: CompileErrorType::SyntaxError(String::from("Unreachable")),
                location: ast.location,
            }),
        }
    }

    fn compile_range(&self, expr: &ast::Expression) -> CompileResult<(BigUint, BigUint)> {
        if let ast::ExpressionType::Range { start, end } = &expr.node {
            Ok((start.clone(), end.clone()))
        } else {
            Err(CompileError {
                error: CompileErrorType::SyntaxError(String::from("Range Compile Error")),
                location: expr.location,
            })
        }
    }

    fn compile_block(&mut self, expr: &ast::Expression) -> CompileResult<Block> {
        if let ast::ExpressionType::CompoundExpression {
            statements,
            return_value,
        } = &expr.node
        {
            self.context.add_block();
            for statement in statements {
                self.compile_stmt(statement)?;
            }
            if let Some(returns) = return_value {
                self.compile_expr(returns)?;
            }
            Ok(self.context.pop_block())
        } else {
            Err(CompileError {
                error: CompileErrorType::SyntaxError(String::from("Block Compile Error")),
                location: expr.location,
            })
        }
    }

    fn look_contract(&mut self, name: &str) {
        self.contracts.insert(name.to_string(), Contract::new());
        self.context.current_contract = Some(name.to_string());
    }

    fn add_function(&mut self, name: &str, params: Vec<Symbol>, ret: Type, block: Block) {
        if let Some(contract_name) = self.context.current_contract.clone() {
            self.context.current_function = Some(name.to_string());
            self.contracts
                .get_mut(&contract_name)
                .unwrap()
                .functions
                .insert(name.to_string(), Function::new(params, ret, block));
        }
    }
}
