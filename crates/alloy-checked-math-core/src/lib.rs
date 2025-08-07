#![feature(try_trait_v2)]

use alloy_primitives::aliases::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckedMathError {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Checked<T> {
    Ok(T),
    Err(CheckedMathError),
}

impl<T> std::ops::FromResidual for Checked<T> {
    fn from_residual(residual: Result<std::convert::Infallible, CheckedMathError>) -> Self {
        match residual {
            Ok(_) => unsafe { std::hint::unreachable_unchecked() },
            Err(err) => Checked::Err(err),
        }
    }
}

impl<T> std::ops::Try for Checked<T> {
    type Output = T;
    type Residual = Result<std::convert::Infallible, CheckedMathError>;

    fn from_output(output: Self::Output) -> Self {
        Checked::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Checked::Ok(value) => std::ops::ControlFlow::Continue(value),
            Checked::Err(err) => std::ops::ControlFlow::Break(Result::Err(err)),
        }
    }
}

macro_rules! impl_checked_math {
    ($($ty:ty),*) => { $(
        impl std::ops::Add for Checked<$ty> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                self?.checked_add(other?).map(Checked::Ok).unwrap_or_else(|| Checked::Err(CheckedMathError::Add))
            }
        }

        impl std::ops::Sub for Checked<$ty> {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                self?.checked_sub(other?).map(Checked::Ok).unwrap_or_else(|| Checked::Err(CheckedMathError::Sub))
            }
        }

        impl std::ops::Mul for Checked<$ty> {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                self?.checked_mul(other?).map(Checked::Ok).unwrap_or_else(|| Checked::Err(CheckedMathError::Mul))
            }
        }

        impl std::ops::Div for Checked<$ty> {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                self?.checked_div(other?).map(Checked::Ok).unwrap_or_else(|| Checked::Err(CheckedMathError::Div))
            }
        }

        impl std::ops::Rem for Checked<$ty> {
            type Output = Self;

            fn rem(self, other: Self) -> Self {
                self?.checked_rem(other?).map(Checked::Ok).unwrap_or_else(|| Checked::Err(CheckedMathError::Rem))
            }
        }

        impl std::ops::Neg for Checked<$ty> {
            type Output = Self;

            fn neg(self) -> Self {
                self?.checked_neg().map(Checked::Ok).unwrap_or_else(|| Checked::Err(CheckedMathError::Neg))
            }
        }
    )* }
}

impl_checked_math!(U0, U1, U8, U16, U24, U32, U40, U48, U56, U64, U72, U80, U88, U96, U104, U112, U120, U128, U136, U144, U152, U160, U168, U176, U184, U192, U200, U208, U216, U224, U232, U240, U248, U256, U512);
impl_checked_math!(I0, I1, I8, I16, I24, I32, I40, I48, I56, I64, I72, I80, I88, I96, I104, I112, I120, I128, I136, I144, I152, I160, I168, I176, I184, I192, I200, I208, I216, I224, I232, I240, I248, I256, I512);
impl_checked_math!(u8, u16, u32, u64, u128, usize);
impl_checked_math!(i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    fn l<T: std::str::FromStr<Err: std::fmt::Debug>>(s: &str) -> Checked<T> {
        Checked::Ok(s.parse::<T>().unwrap())
    }

    #[test]
    fn test_checked_add() {
        {
            type T = u8;
            assert_eq!(l::<T>("1") + l::<T>("2"), l::<T>("3"));
            assert_eq!(l::<T>("255") + l::<T>("1"), Checked::Err(CheckedMathError::Add));
        }

        {
            type T = U256;
            assert_eq!(l::<T>("1") + l::<T>("2"), l::<T>("3"));
            assert_eq!(Checked::Ok(U256::MAX) + l::<T>("1"), Checked::Err(CheckedMathError::Add));
        }

        {
            type T = I8;
            assert_eq!(l::<T>("64") + l::<T>("63"), l::<T>("127"));
            assert_eq!(l::<T>("-64") + l::<T>("63"), l::<T>("-1"));
            assert_eq!(l::<T>("64") + l::<T>("64"), Checked::Err(CheckedMathError::Add));
        }
    }

    #[test]
    fn test_checked_sub() {
        {
            type T = U8;
            assert_eq!(l::<T>("3") - l::<T>("2"), l::<T>("1"));
            assert_eq!(l::<T>("0") - l::<T>("1"), Checked::Err(CheckedMathError::Sub));
        }

        {
            type T = U256;
            assert_eq!(l::<T>("3") - l::<T>("2"), l::<T>("1"));
            assert_eq!(Checked::Ok(U256::MIN) - l::<T>("1"), Checked::Err(CheckedMathError::Sub));
        }

        {
            type T = I8;
            assert_eq!(l::<T>("64") - l::<T>("63"), l::<T>("1"));
            assert_eq!(l::<T>("-64") - l::<T>("64"), l::<T>("-128"));
            assert_eq!(l::<T>("-64") - l::<T>("65"), Checked::Err(CheckedMathError::Sub));
        }
    }

    #[test]
    fn test_checked_mul() {
        {
            type T = U8;
            assert_eq!(l::<T>("3") * l::<T>("2"), l::<T>("6"));
            assert_eq!(l::<T>("32") * l::<T>("8"), Checked::Err(CheckedMathError::Mul));
        }
    }

    #[test]
    fn test_checked_div() {
        {
            type T = U8;
            assert_eq!(l::<T>("6") / l::<T>("2"), l::<T>("3"));
            assert_eq!(l::<T>("0") / l::<T>("1"), l::<T>("0"));
            assert_eq!(l::<T>("1") / l::<T>("0"), Checked::Err(CheckedMathError::Div));
        }
    }

    #[test]
    fn test_checked_rem() {
        {
            type T = U8;
            assert_eq!(l::<T>("5") % l::<T>("2"), l::<T>("1"));
            assert_eq!(l::<T>("0") % l::<T>("1"), l::<T>("0"));
            assert_eq!(l::<T>("1") % l::<T>("0"), Checked::Err(CheckedMathError::Rem));
        }
    }
}
