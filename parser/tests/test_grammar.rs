use ziraffe_parser::parser;

#[test]
fn test_if_expression_parser() {
    assert!(parser::parse_expression("if a < 2 { 1 } else { 0 }").is_ok());
    assert!(parser::parse_expression("b = if a < 2 { 1 } else { 0 }").is_ok());
}

#[test]
fn test_for_expression_parser() {
    assert!(parser::parse_expression("for i in 1..10 { a = a + 1; }").is_ok());
    assert!(parser::parse_expression("for _ in 0..9 { b = b + 2; }").is_ok());
}

#[test]
fn test_assign_expression_parser() {
    assert!(parser::parse_expression("a = \"abc\"").is_ok());
    assert!(parser::parse_expression("a = b").is_ok());
    assert!(parser::parse_expression("a = 1").is_ok());
}

#[test]
fn test_init_statement_parser() {
    assert!(parser::parse_statement("string url = \"abc\"").is_ok());
    assert!(parser::parse_statement("uint a = b").is_ok());
    assert!(parser::parse_statement("uint a = 1").is_ok());
}
