use cloudy::cloudylang::Interpreter;

fn test_eq(input: &str, expected: &str) {
    let result = Interpreter::new().interpret(input, "<stdin>").unwrap();
    assert_eq!(result.dbg(), expected);
}

fn test_ne_cdy(input_1: &str, input_2: &str) {
    let result_1 = Interpreter::new().interpret(input_1, "<stdin>").unwrap();
    let result_2 = Interpreter::new().interpret(input_2, "<stdin>").unwrap();
    assert_ne!(result_1.dbg(), result_2.dbg());
}

fn test_eq_cdy(input_1: &str, input_2: &str) {
    let result_1 = Interpreter::new().interpret(input_1, "<stdin>").unwrap();
    let result_2 = Interpreter::new().interpret(input_2, "<stdin>").unwrap();
    assert_eq!(result_1.dbg(), result_2.dbg());
}

#[test]
fn single_int_test() {
    test_eq("1", "1");
    test_eq("-1", "-1");
    test_eq("12", "12");
    test_eq("-12", "-12");
}

#[test]
fn single_float_test() {
    test_eq("1.0", "1");
    test_eq("-1.0", "-1");
    test_eq("12.23", "12.23");
    test_eq("-12.230", "-12.23");
}

#[test]
fn zero_test() {
    test_eq("0", "0");
    test_eq("-0", "0");
    test_eq("+0", "0");
    test_eq("-0.0", "0");
    test_eq("+0.0", "0");
    test_eq("0000", "0");
    test_eq("-0000", "0");
}

#[test]
fn unary_plus_test() {
    test_eq("+1", "1");
    test_eq("+-1", "-1");
    test_eq("+12", "12");
    test_eq("+-12", "-12");
    test_eq("+1.0", "1");
    test_eq("+-1.0", "-1");
    test_eq("+12.23", "12.23");
    test_eq("+-12.230", "-12.23");
}

#[test]
fn unary_minus_test() {
    test_eq("-1", "-1");
    test_eq("--1", "1");
    test_eq("-12", "-12");
    test_eq("--12", "12");
    test_eq("-1.0", "-1");
    test_eq("--1.0", "1");
    test_eq("-12.23", "-12.23");
    test_eq("--12.230", "12.23");
}

#[test]
fn binary_addition_test() {
    test_eq("1 + 1", &format!("{}", 1 + 1));
    test_eq("1 + -1", &format!("{}", 1 + -1));
    test_eq("1 + 12", &format!("{}", 1 + 12));
    test_eq("1 + -12", &format!("{}", 1 + -12));
    test_eq("1.0 + 1", &format!("{}", 1.0 + 1.0));
    test_eq("1.0 + -1", &format!("{}", 1.0 + -1.0));
    test_eq("1.0 + 12", &format!("{}", 1.0 + 12.0));
    test_eq("1.0 + -12", &format!("{}", 1.0 + -12.0));
    test_eq("12 + 1", &format!("{}", 12 + 1));
    test_eq("12 + -1", &format!("{}", 12 + -1));
    test_eq("12 + 12", &format!("{}", 12 + 12));
    test_eq("12 + -12", &format!("{}", 12 + -12));
    test_eq("12.23 + 1", &format!("{}", 12.23 + 1.0));
    test_eq("12.23 + -1", &format!("{}", 12.23 + -1.0));
    test_eq("12.23 + 12", &format!("{}", 12.23 + 12.0));
    test_eq("12.23 + -12", &format!("{}", 12.23 + -12.0));
}

#[test]
fn binary_subtraction_test() {
    test_eq("1 - 1", &format!("{}", 1 - 1));
    test_eq("1 - -1", &format!("{}", 1 - -1));
    test_eq("1 - 12", &format!("{}", 1 - 12));
    test_eq("1 - -12", &format!("{}", 1 - -12));
    test_eq("1.0 - 1", &format!("{}", 1.0 - 1.0));
    test_eq("1.0 - -1", &format!("{}", 1.0 - -1.0));
    test_eq("1.0 - 12", &format!("{}", 1.0 - 12.0));
    test_eq("1.0 - -12", &format!("{}", 1.0 - -12.0));
    test_eq("12 - 1", &format!("{}", 12 - 1));
    test_eq("12 - -1", &format!("{}", 12 - -1));
    test_eq("12 - 12", &format!("{}", 12 - 12));
    test_eq("12 - -12", &format!("{}", 12 - -12));
    test_eq("12.23 - 1", &format!("{}", 12.23 - 1.0));
    test_eq("12.23 - -1", &format!("{}", 12.23 - -1.0));
    test_eq("12.23 - 12", &format!("{}", 12.23 - 12.0));
    test_eq("12.23 - -12", &format!("{}", 12.23 - -12.0));
}

#[test]
fn binary_multiplication_test() {
    test_eq("1 * 1", &format!("{}", 1 * 1));
    test_eq("1 * -1", &format!("{}", 1 * -1));
    test_eq("1 * 12", &format!("{}", 1 * 12));
    test_eq("1 * -12", &format!("{}", 1 * -12));
    test_eq("1.0 * 1", &format!("{}", 1.0 * 1.0));
    test_eq("1.0 * -1", &format!("{}", 1.0 * -1.0));
    test_eq("1.0 * 12", &format!("{}", 1.0 * 12.0));
    test_eq("1.0 * -12", &format!("{}", 1.0 * -12.0));
    test_eq("12 * 1", &format!("{}", 12 * 1));
    test_eq("12 * -1", &format!("{}", 12 * -1));
    test_eq("12 * 12", &format!("{}", 12 * 12));
    test_eq("12 * -12", &format!("{}", 12 * -12));
    test_eq("12.23 * 1", &format!("{}", 12.23 * 1.0));
    test_eq("12.23 * -1", &format!("{}", 12.23 * -1.0));
    test_eq("12.23 * 12", &format!("{}", 12.23 * 12.0));
    test_eq("12.23 * -12", &format!("{}", 12.23 * -12.0));
}

#[test]
fn binary_division_test() {
    test_eq("1 / 1", &format!("{}", 1 / 1));
    test_eq("1 / -1", &format!("{}", 1 / -1));
    test_eq("1 / 12", &format!("{}", 1.0 / 12.0));
    test_eq("1 / -12", &format!("{}", 1.0 / -12.0));
    test_eq("1.0 / 1", &format!("{}", 1.0 / 1.0));
    test_eq("1.0 / -1", &format!("{}", 1.0 / -1.0));
    test_eq("1.0 / 12", &format!("{}", 1.0 / 12.0));
    test_eq("1.0 / -12", &format!("{}", 1.0 / -12.0));
    test_eq("12 / 1", &format!("{}", 12 / 1));
    test_eq("12 / -1", &format!("{}", 12 / -1));
    test_eq("12 / 12", &format!("{}", 12 / 12));
    test_eq("12 / -12", &format!("{}", 12 / -12));
    test_eq("12.23 / 1", &format!("{}", 12.23 / 1.0));
    test_eq("12.23 / -1", &format!("{}", 12.23 / -1.0));
    test_eq("12.23 / 12", &format!("{}", 12.23 / 12.0));
    test_eq("12.23 / -12", &format!("{}", 12.23 / -12.0));
}

#[test]
fn binary_modulo_test() {
    test_eq("1 % 1", &format!("{}", 1 % 1));
    test_eq("1 % -1", &format!("{}", 1 % -1));
    test_eq("1 % 12", &format!("{}", 1 % 12));
    test_eq("1 % -12", &format!("{}", 1 % -12));
    test_eq("1.0 % 1", &format!("{}", 1.0 % 1.0));
    test_eq("1.0 % -1", &format!("{}", 1.0 % -1.0));
    test_eq("1.0 % 12", &format!("{}", 1.0 % 12.0));
    test_eq("1.0 % -12", &format!("{}", 1.0 % -12.0));
    test_eq("12 % 1", &format!("{}", 12 % 1));
    test_eq("12 % -1", &format!("{}", 12 % -1));
    test_eq("12 % 12", &format!("{}", 12 % 12));
    test_eq("12 % -12", &format!("{}", 12 % -12));
    test_eq("12.23 % 1", &format!("{}", 12.23 % 1.0));
    test_eq("12.23 % -1", &format!("{}", 12.23 % -1.0));
    test_eq("12.23 % 12", &format!("{}", 12.23 % 12.0));
    test_eq("12.23 % -12", &format!("{}", 12.23 % -12.0));
}

#[test]
fn binary_exponentiation_test() {
    test_eq("1 ^ 1", &format!("{}", 1i64.pow(1)));
    test_eq("1 ^ -1", &format!("{}", 1f64.powf(-1.0)));
    test_eq("1 ^ 12", &format!("{}", 1i64.pow(12)));
    test_eq("1 ^ -12", &format!("{}", 1f64.powf(-12.0)));
    test_eq("1.0 ^ 1", &format!("{}", 1.0f64.powf(1.0)));
    test_eq("1.0 ^ -1", &format!("{}", 1.0f64.powf(-1.0)));
    test_eq("1.0 ^ 12", &format!("{}", 1.0f64.powf(12.0)));
    test_eq("1.0 ^ -12", &format!("{}", 1.0f64.powf(-12.0)));
    test_eq("12 ^ 1", &format!("{}", 12i64.pow(1)));
    test_eq("12 ^ -1", &format!("{}", 12f64.powf(-1.0)));
    test_eq("12 ^ 12", &format!("{}", 12i64.pow(12)));
    test_eq("12 ^ -12", &format!("{}", 12f64.powf(-12.0)));
    test_eq("12.23 ^ 1", &format!("{}", 12.23f64.powf(1.0)));
    test_eq("12.23 ^ -1", &format!("{}", 12.23f64.powf(-1.0)));
    test_eq("12.23 ^ 12", &format!("{}", 12.23f64.powf(12.0)));
    test_eq("12.23 ^ -12", &format!("{}", 12.23f64.powf(-12.0)));
}

#[test]
fn parenthesis_test() {
    test_ne_cdy("5 * 5 + 5", "5 * (5 + 5)");
    test_ne_cdy("2 ^ 3 ^ 4", "(2 ^ 3) ^ 4");
    test_eq_cdy("5 * 5 + 5", "(5 * 5) + 5");
    test_eq_cdy("2 ^ 3 ^ 4", "2 ^ (3 ^ 4)");
    test_eq_cdy("1 - 1 - 1", "(1 - 1) - 1");
}
