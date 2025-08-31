#![allow(unused)]

use alloy_primitives::aliases::*;
use std::str::FromStr as _;

use alloy_checked_math::Checked;
use alloy_checked_math::{unchecked, checked, checked_fn, unchecked_fn, CheckedMathError};

#[derive(Debug, Clone, PartialEq, derive_more::From)]
pub enum Error {
    CheckedMathError(CheckedMathError),
}

fn id(x: I32) -> Result<I32, Error> {
    Ok(x)
}

pub fn example(y: I32) -> Result<I32, Error> {
    let x = unchecked! { I32::ONE + I32::ONE };
    let z = checked! { I32::ONE + x + -id(y + y)? };
    return Ok(z);
}

mod qwe {

    use super::*;

    fn f(y: I32) {
        let x = unchecked! { true && (I32::ONE + (y + y) == I32::ZERO) };
    }

    struct X;
    impl X {
        fn g() {
            let x = unchecked! { -I32::from_str("2").unwrap() };
            let y = unchecked! { I32::from_str("3").unwrap() + I32::from_str("4").unwrap() };
            let z = unchecked! { y + I32::from_str("5").unwrap() };
        }
    }
}

#[checked_fn]
fn div(x: i32, y: i32) -> Result<i32, Error> {
    return Ok(x / y);
}

#[unchecked_fn]
fn div_unchecked(x: i32, y: i32) -> i32 {
    x / y
}

#[checked_fn]
fn div_assign(mut x: i32, y: i32) -> Result<i32, Error> {
    x /= y;
    Ok(x)
}

#[cfg(test)]
#[test]
fn example_test() {
    assert_eq!(example(I32::from_str("3").unwrap()), Ok(I32::from_str("-3").unwrap()));

    assert_eq!(div(10, 2), Ok(5));
    assert_eq!(div(10, 0), Err(Error::CheckedMathError(CheckedMathError::Div)));

    assert_eq!(div_assign(10, 2), Ok(5));
    assert_eq!(div_assign(10, 0), Err(Error::CheckedMathError(CheckedMathError::Div)));
}
