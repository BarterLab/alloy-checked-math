#![allow(unused)]

use alloy_primitives::aliases::*;
use std::str::FromStr as _;

use alloy_checked_math_core::CheckedMathError;
use alloy_checked_math_macro::{unchecked, checked};

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

#[cfg(test)]
#[test]
fn example_test() {
    assert_eq!(example(I32::from_str("3").unwrap()), Ok(I32::from_str("-3").unwrap()));
}
