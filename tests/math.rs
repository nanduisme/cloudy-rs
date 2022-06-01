use cloudy::cloudylang::Interpreter;

fn test(input: &str, expected: &str) {
    let result = Interpreter::new().interpret(input, "<stdin>").unwrap();
    assert_eq!(result.debug(), expected);
}

// single int tests
#[test]
fn int_test_1() {
    let input = "1";
    let expected = "1";
    test(input, expected);
}

#[test]
fn int_test_2() {
    let input = "123";
    let expected = "123";
    test(input, expected);
}

#[test]
fn int_test_3() {
    let input = "123456789";
    let expected = "123456789";
    test(input, expected);
}

#[test]
fn int_test_4() {
    let input = "-1";
    let expected = "-1";
    test(input, expected);
}

#[test]
fn int_test_5() {
    let input = "-123";
    let expected = "-123";
    test(input, expected);
}

#[test]
fn int_test_6() {
    let input = "-123456789";
    let expected = "-123456789";
    test(input, expected);
}

#[test]
fn int_test_7() {
    let input = "0000";
    let expected = "0";
    test(input, expected);
}

#[test]
fn int_test_8() {
    let input = "-00000";
    let expected = "0";
    test(input, expected);
}

// single float tests
#[test]
fn float_test_1() {
    let input = "1.0";
    let expected = "1";
    test(input, expected);
}

#[test]
fn float_test_2() {
    let input = "1.1";
    let expected = "1.1";
    test(input, expected);
}

#[test]
fn float_test_3() {
    let input = "1.123456789";
    let expected = "1.123456789";
    test(input, expected);
}

#[test]
fn float_test_4() {
    let input = "-1.0";
    let expected = "-1";
    test(input, expected);
}

#[test]
fn float_test_5() {
    let input = "-1.1";
    let expected = "-1.1";
    test(input, expected);
}

#[test]
fn float_test_6() {
    let input = "-1.123456789";
    let expected = "-1.123456789";
    test(input, expected);
}

#[test]
fn float_test_7() {
    let input = "0.0";
    let expected = "0";
    test(input, expected);
}

#[test]
fn float_test_8() {
    let input = "-0.0";
    let expected = "0";
    test(input, expected);
}

// int addition tests
#[test]
fn int_add_test_1() {
    let input = "1 + 1";
    let expected = "2";
    test(input, expected);
}

#[test]
fn int_add_test_2() {
    let input = "123 + 456";
    let expected = "579";
    test(input, expected);
}

#[test]
fn int_add_test_3() {
    let input = "123456789 + 123456789";
    let expected = "246913578";
    test(input, expected);
}

#[test]
fn int_add_test_4() {
    let input = "123456789 + -123456789";
    let expected = "0";
    test(input, expected);
}

#[test]
fn int_add_test_5() {
    let input = "-123456789 + 123456789";
    let expected = "0";
    test(input, expected);
}

#[test]
fn int_add_test_6() {
    let input = "-123456789 + -123456789";
    let expected = "-246913578";
    test(input, expected);
}

// int subtraction tests
#[test]
fn int_sub_test_1() {
    let input = "1 - 1";
    let expected = "0";
    test(input, expected);
}

#[test]
fn int_sub_test_2() {
    let input = "123 - 456";
    let expected = "-333";
    test(input, expected);
}

#[test]
fn int_sub_test_3() {
    let input = "123456789 - 123456789";
    let expected = "0";
    test(input, expected);
}

#[test]
fn int_sub_test_4() {
    let input = "123456789 - -123456789";
    let expected = "246913578";
    test(input, expected);
}

#[test]
fn int_sub_test_5() {
    let input = "-123456789 - 123456789";
    let expected = "-246913578";
    test(input, expected);
}

#[test]
fn int_sub_test_6() {
    let input = "-123456789 - -123456789";
    let expected = "0";
    test(input, expected);
}

// int multiplication tests
#[test]
fn int_mul_test_1() {
    let input = "1 * 1";
    let expected = "1";
    test(input, expected);
}

#[test]
fn int_mul_test_2() {
    let input = "123 * 456";
    let expected = "56088";
    test(input, expected);
}

#[test]
fn int_mul_test_3() {
    let input = "123456789 * 123456789";
    let expected = "15241578750190520";
    test(input, expected);
}

#[test]
fn int_mul_test_4() {
    let input = "123456789 * -123456789";
    let expected = "-15241578750190520";
    test(input, expected);
}

#[test]
fn int_mul_test_5() {
    let input = "-123456789 * 123456789";
    let expected = "-15241578750190520";
    test(input, expected);
}

#[test]
fn int_mul_test_6() {
    let input = "-123456789 * -123456789";
    let expected = "15241578750190520";
    test(input, expected);
}

// int division tests
#[test]
fn int_div_test_1() {
    let input = "1 / 1";
    let expected = "1";
    test(input, expected);
}

#[test]
fn int_div_test_2() {
    let input = "123 / 456";
    let expected = "0.26973684210526316";
    test(input, expected);
}

#[test]
fn int_div_test_3() {
    let input = "123456789 / 123456789";
    let expected = "1";
    test(input, expected);
}

#[test]
fn int_div_test_4() {
    let input = "123456789 / -123456789";
    let expected = "-1";
    test(input, expected);
}

#[test]
fn int_div_test_5() {
    let input = "-123456789 / 123456789";
    let expected = "-1";
    test(input, expected);
}

#[test]
fn int_div_test_6() {
    let input = "-123456789 / -123456789";
    let expected = "1";
    test(input, expected);
}
