#![allow(unused)]

use checked_math::CheckedError;
use checked_macro::{unchecked, checked};

#[derive(Debug, Clone, PartialEq, derive_more::From)]
pub enum Error {
    CheckedError(CheckedError),
}

fn id(x: i32) -> Result<i32, Error> {
    Ok(x)
}

pub fn example(y: i32) -> Result<i32, Error> {
    let x = 1 + 1;
    let z = checked! { 1 + x + -id(y + y)? };
    return Ok(z);
}

mod qwe {
    use super::*;

    fn f(y: i32) {
        let x = 1 + (y + y);
    }

    struct X;
    impl X {
        fn g() {
            let y = 3 + 4;
            let z = unchecked! { y + 5 };
        }
    }
}

#[cfg(test)]
#[test]
fn example_test() {
    assert_eq!(example(3), Ok(-3));
    assert_eq!(example(0), Err(Error::CheckedError(CheckedError::ZeroIsNotAllowed)));
}
