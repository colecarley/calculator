use calculator::{
    interpreter::interpreter::Interpreter, lexer::lexer::Lexer, parser::parser::Parser,
    token::token::Value,
};

fn evaluate(input: String) -> Value {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    let root = parser.parse();

    let mut interpreter = Interpreter::new();
    let result = interpreter.evaluate(root);
    return result;
}

#[test]
fn test_add() {
    let input = "let int x = 6; let int y = 6; x + y;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_sub() {
    let input = "let int x = 24; let int y = 12; x - y;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_div() {
    let input = "let int x = 24; let int y = 2; x / y;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_mult() {
    let input = "let int x = 6; let int y = 2; x * y;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_parens() {
    let input = "(1 + 2) * 3;".to_string();
    assert_eq!(evaluate(input), Value::Number(9));
}

#[test]
fn test_assignment() {
    let input = "let int x = 6; x;".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_equality_with_numbers() {
    let input = "let int x = 6; let int y = 6; x == y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_equality_with_strings() {
    let input = "let str x = \"hello\"; let str y = \"hello\"; x == y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_equality_with_booleans() {
    let input = "let bool x = true; let bool y = true; x == y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_inequality_with_numbers() {
    let input = "let int x = 6; let int y = 12; x != y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_inequality_with_strings() {
    let input = "let str x = \"hello\"; let str y = \"world\"; x != y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_inequality_with_booleans() {
    let input = "let bool x = true; let bool y = false; x != y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_greater_than() {
    let input = "let int x = 12; let int y = 6; x > y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_less_than() {
    let input = "let int x = 6; let int y = 12; x < y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_greater_than_or_equal_to() {
    let input = "let int x = 12; let int y = 6; x >= y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_less_than_or_equal_to() {
    let input = "let int x = 6; let int y = 12; x <= y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_assignment_with_expression() {
    let input = "let int x = 6; let int y = x + 6; y;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_assignment_with_boolean() {
    let input = "let int x = 6; let bool y = x == 6; y;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_assignment_with_string() {
    let input = "let str x = \"hello\"; x;".to_string();
    assert_eq!(evaluate(input), Value::String("hello".to_string()));
}

#[test]
fn test_reassignment() {
    let input = "let int x = 6; x = 12; x;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_reassignment_with_expression() {
    let input = "let int x = 6; x = x + 6; x;".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_reassignment_with_boolean() {
    let input = "let bool x = false; x = true; x;".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]

fn test_reassignment_with_string() {
    let input = "let str x = \"hello\"; x = x + \" world\"; x;".to_string();
    assert_eq!(evaluate(input), Value::String("hello world".to_string()));
}

#[test]
fn test_assignment_with_list() {
    let input = "let list x = [1, 2, 3]; x;".to_string();
    assert_eq!(
        evaluate(input),
        Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(3)])
    );
}

#[test]
fn test_assignment_with_empty_list() {
    let input = "let list x = []; x;".to_string();
    assert_eq!(evaluate(input), Value::List(vec![]));
}

#[test]
fn test_function_declaration() {
    let input = "funk add(int x, int y) { x + y; } add(6, 6);".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_function_declaration_with_recursion() {
    let input =
        "funk factorial(int x) { if (x == 0) { 1; } else { x * factorial(x - 1); } } factorial(5);"
            .to_string();
    assert_eq!(evaluate(input), Value::Number(120));
}

#[test]
fn test_function_declaration_with_nested_function() {
    let input =
        "funk add(int x, int y) { funk add2(int x, int y) { x + y; } add2(x, y); } add(6, 6);"
            .to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_function_with_early_return() {
    let input =
        "funk add(int x, int y) { if (x == 6) { return 6; } x + y; } add(6, 6);".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]

fn test_function_with_early_return_in_nested_function() {
    let input = "funk add(int x, int y) { funk add2(int x, int y) { if (x == 6) { return 6; } x + y; } add2(x, y); } add(6, 6);".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_function_with_return() {
    let input = "funk add(int x, int y) { return x + y; } add(6, 6);".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_function_with_return_in_nested_function() {
    let input =
        "funk add(int x, int y) { funk add2(int x, int y) { return x + y; } add2(x, y); } add(6, 6);".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_if_statement() {
    let input = "if (6 == 6) { 6; } else { 12; }".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_if_statement_without_else() {
    let input = "if (6 == 6) { 6; }".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_if_statement_with_nested_if() {
    let input = "if (6 == 6) { if (6 == 6) { 6; } }".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_higher_order_function() {
    let input =
        "funk add(int x, int y) { x + y; } funk apply(function f, int x, int y) { f(x, y); } apply(add, 6, 6);".to_string();
    assert_eq!(evaluate(input), Value::Number(12));
}

#[test]
fn test_indexing() {
    let input = "let list x = [1, 2, 3]; x[0];".to_string();
    assert_eq!(evaluate(input), Value::Number(1));
}

#[test]
fn test_indexing_with_expression() {
    let input = "let list x = [1, 2, 3]; x[1 + 1];".to_string();
    assert_eq!(evaluate(input), Value::Number(3));
}

#[test]
fn test_indexing_with_function_call() {
    let input = "funk one() { 1 }; let list x = [1,2,3]; x[one()]".to_string();
    assert_eq!(evaluate(input), Value::Number(2));
}

#[test]
fn test_indexing_function_result() {
    let input = "funk create_list() { [1,2,3] }; create_list()[0]".to_string();
    assert_eq!(evaluate(input), Value::Number(1));
}

#[test]
fn test_single_line_comment() {
    let input = "let int x = 6; // this is a comment\nx;".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_multi_line_comment() {
    let input = "let int x = 6; /* this is a\nmulti-line\ncomment */ x;".to_string();
    assert_eq!(evaluate(input), Value::Number(6));
}

#[test]
fn test_string_concatenation() {
    let input = "\"hello\" + \" \" + \"world\"".to_string();
    assert_eq!(evaluate(input), Value::String("hello world".to_string()));
}

#[test]
fn test_list_concatenation() {
    let input = "[1, 2] + [3, 4]".to_string();
    assert_eq!(
        evaluate(input),
        Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
            Value::Number(4)
        ])
    );
}

#[test]
fn test_head() {
    let input = "head([1, 2, 3])".to_string();
    assert_eq!(evaluate(input), Value::Number(1));
}

#[test]
fn test_tail() {
    let input = "tail([1, 2, 3])".to_string();
    assert_eq!(
        evaluate(input),
        Value::List(vec![Value::Number(2), Value::Number(3)])
    );
}

#[test]
fn test_len() {
    let input = "len([1, 2, 3])".to_string();
    assert_eq!(evaluate(input), Value::Number(3));
}

#[test]
fn test_type_with_number() {
    let input = "type(6)".to_string();
    assert_eq!(evaluate(input), Value::String("number".to_string()));
}

#[test]
fn test_type_with_string() {
    let input = "type(\"hello\")".to_string();
    assert_eq!(evaluate(input), Value::String("string".to_string()));
}

#[test]
fn test_type_with_boolean() {
    let input = "type(true)".to_string();
    assert_eq!(evaluate(input), Value::String("bool".to_string()));
}

#[test]
fn test_type_with_list() {
    let input = "type([1, 2, 3])".to_string();
    assert_eq!(evaluate(input), Value::String("list".to_string()));
}

#[test]
fn test_type_with_function() {
    let input = "funk add(int x, int y) { x + y; } type(add)".to_string();
    assert_eq!(evaluate(input), Value::String("function".to_string()));
}

#[test]
fn test_type_with_variable() {
    let input = "let int x = 6; type(x)".to_string();
    assert_eq!(evaluate(input), Value::String("number".to_string()));
}

#[test]
fn test_is_bool() {
    let input = "is_bool(true)".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_is_number() {
    let input = "is_number(6)".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_is_string() {
    let input = "is_string(\"hello\")".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_is_list() {
    let input = "is_list([1, 2, 3])".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

#[test]
fn test_is_function() {
    let input = "funk add(int x, int y) { x + y; } is_function(add)".to_string();
    assert_eq!(evaluate(input), Value::Boolean(true));
}

// misc

#[test]
fn test_factorial() {
    let input =
        "funk factorial(int x) { if (x == 0) { 1; } else { x * factorial(x - 1); } } factorial(5);"
            .to_string();
    assert_eq!(evaluate(input), Value::Number(120));
}

#[test]
fn test_fibonacci() {
    let input = "funk fibonacci(int x) { if (x == 0) { 0; } else { if (x == 1) { 1; } else { fibonacci(x - 1) + fibonacci(x - 2); } } } fibonacci(10);".to_string();
    assert_eq!(evaluate(input), Value::Number(55));
}

#[test]
fn test_fibonacci_with_tail_call_optimization() {
    let input = "funk fibonacci(int x) { funk fib(int x, int a, int b) { if (x == 0) { a; } else { fib(x - 1, b, a + b); } } fib(x, 0, 1); } fibonacci(10);".to_string();
    assert_eq!(evaluate(input), Value::Number(55));
}
