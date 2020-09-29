use crate::error::{CompileError, CompileErrorType};
use crate::instruction::{Block, Instruction};
use indexmap::map::IndexMap;
use ziraffe_parser::ast;
use ziraffe_parser::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    URL,
    JSON,
    Uint,
    Bool,
    Address,
    // only support equality zkp.
    String,
    None,
    Undefined,
}

impl Type {
    pub fn get_type(typ: &ast::Type) -> Self {
        match typ {
            ast::Type::URL => Type::URL,
            ast::Type::JSON => Type::JSON,
            ast::Type::Uint => Type::Uint,
            ast::Type::Bool => Type::Bool,
            ast::Type::String => Type::String,
            ast::Type::Address => Type::Address,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: String,
    pub num: u32,
    pub typ: Type,
}

type SymbolResult<T> = Result<T, CompileError>;

impl Symbol {
    pub fn temp_symbol(context: &mut Context) -> Self {
        Symbol {
            id: String::from("_"),
            num: context.numbering_temp(),
            typ: Type::None,
        }
    }
    pub fn literal_symbol(literal: String) -> Self {
        Symbol {
            id: literal,
            num: 0,
            typ: Type::String,
        }
    }

    pub fn result_symbol(
        context: &mut Context,
        a: Symbol,
        b: Symbol,
        loc: Location,
    ) -> SymbolResult<Self> {
        println!("{:#?} a : b {:#?}", a.typ, b.typ);
        if a.typ == b.typ {
            let typ = b.typ;
            Ok(Symbol {
                id: String::from(""),
                num: context.numbering_temp(),
                typ,
            })
        } else {
            Err(CompileError {
                error: CompileErrorType::TypeError(String::from("Binary operation Type Error")),
                location: loc,
            })
        }
    }
}

#[derive(Default)]
pub struct SymbolTable {
    pub symbols: IndexMap<String, Symbol>,
}

impl SymbolTable {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct Function {
    params: Vec<Symbol>,
    codes: Block,
    ret: Type,
}

impl Function {
    pub fn new(params: Vec<Symbol>, ret: Type, codes: Block) -> Self {
        Function { params, codes, ret }
    }
}

#[derive(Debug, Default)]
pub struct Contract {
    pub member: IndexMap<String, Symbol>,
    pub functions: IndexMap<String, Function>,
}

impl Contract {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_function(&mut self, name: String, func: Function) {
        self.functions.insert(name, func);
    }
}

#[derive(Default)]
pub struct Context {
    pub tables: Vec<SymbolTable>,
    pub current_codes: Vec<Block>,
    pub current_contract: Option<String>,
    pub current_function: Option<String>,
    pub temp_number: u32,
    pub is_member: bool,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.current_codes
            .last_mut()
            .unwrap()
            .add_instruction(instruction);
    }

    pub fn add_block(&mut self) {
        self.current_codes.push(Block::new());
        self.tables.push(SymbolTable::new());
    }

    pub fn pop_block(&mut self) -> Block {
        self.tables.pop();
        self.current_codes.pop().unwrap()
    }

    pub fn numbering_temp(&mut self) -> u32 {
        self.temp_number += 1;
        self.temp_number
    }

    pub fn get_symbol(&self, name: &str) -> Symbol {
        for table in self.tables.iter().rev() {
            if let Some(symbol) = table.symbols.get(name) {
                return symbol.clone();
            }
        }
        Symbol {
            id: name.to_string(),
            num: 0,
            typ: Type::Undefined,
        }
    }

    pub fn add_symbol(&mut self, name: &str, sym: Symbol) {
        self.tables
            .last_mut()
            .unwrap()
            .symbols
            .insert(name.to_string(), sym);
    }
}
