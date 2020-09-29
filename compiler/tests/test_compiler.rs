use ziraffe_compiler::compiler::compile_program;
use ziraffe_parser::parser;

#[test]
fn test_compile_program() {
    let program = parser::parse_program(
        "contract A { uint b; function f() { uint a = 0; for i in 0..10 { a = a + 1; } } }",
    );
    assert!(program.is_ok());
    let contracts = compile_program(&program.unwrap()).unwrap();
    let contract = contracts.get("A").unwrap();
    let function = contract.functions.get("f").unwrap();
}
