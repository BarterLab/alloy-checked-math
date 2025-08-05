use checked_math::CheckedError;
use checked_macro::checked;

#[derive(Debug, Clone, PartialEq, derive_more::From)]
enum Error {
    CheckedError(CheckedError),
}

fn id(x: i32) -> Result<i32, Error> {
    Ok(x)
}

fn example(y: i32) -> Result<i32, Error> {
    let x = 2;
    let z = checked! { 1 + x + -id(y + y)? };
    return Ok(z);
}

fn main() {
    assert_eq!(example(3), Ok(-3));
    assert_eq!(example(0), Err(Error::CheckedError(CheckedError::ZeroIsNotAllowed)));
}
