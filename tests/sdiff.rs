use sdiff::*;
use std::fmt::Write;

// TODO : Tests use write and string comparison, rather than tree equality.

#[test]
fn test_diff_number_is_0() {
    let val = Expression::lit(3);
    let r = (*val).differentiate();
    let mut output = String::new();
    write!(output, "{}", *r).unwrap();
    assert_eq!(output, "0");
}

#[test]
fn test_diff_x_is_1() {
    let val = Expression::var('x');
    let r = (*val).differentiate();
    let mut output = String::new();
    write!(output, "{}", *r).unwrap();
    assert_eq!(output, "1");
}

#[test]
fn test_diff_3x_is_3() {
    let val = Expression::mul(Expression::lit(3), Expression::var('x'));
    let r = (*val).differentiate();
    let mut output = String::new();
    write!(output, "{}", *r).unwrap();
    assert_eq!(output, "3");
}

#[test]
fn test_diff_xy_is_y() {
    let val = Expression::mul(Expression::var('x'), Expression::var('y'));
    let r = (*val).differentiate();
    let mut output = String::new();
    write!(output, "{}", *r).unwrap();
    assert_eq!(output, "y");
}

#[test]
fn test_diff_y_is_0() {
    let val = Expression::var('x');
    let r = (*val).differentiate();
    let mut output = String::new();
    write!(output, "{}", *r).unwrap();
    assert_eq!(output, "1");
}

#[test]
fn test_diff_x_cubed_is_3_x_squared() {
    let val = Expression::exp(Expression::var('x'), Expression::lit(3));
    let r = (*val).differentiate();
    let mut output = String::new();
    write!(output, "{}", *r).unwrap();
    assert_eq!(output, "3*x^2");
}