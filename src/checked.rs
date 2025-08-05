use std::ops::FromResidual;

#[derive(Debug, Clone, PartialEq)]
pub enum CheckedError {
    ZeroIsNotAllowed,
}

pub enum Checked<T> {
    Ok(T),
    Err(CheckedError),
}

impl<T> FromResidual for Checked<T> {
    fn from_residual(residual: Result<std::convert::Infallible, CheckedError>) -> Self {
        match residual {
            Ok(_) => unsafe { std::hint::unreachable_unchecked() },
            Err(err) => Checked::Err(err),
        }
    }
}

impl<T> std::ops::Try for Checked<T> {
    type Output = T;
    type Residual = Result<std::convert::Infallible, CheckedError>;

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

impl std::ops::Add for Checked<i32> {
    type Output = Checked<i32>;

    fn add(self, other: Checked<i32>) -> Checked<i32> {
        let a = self?;
        let b = other?;

        if a == 0 || b == 0 {
            return Checked::Err(CheckedError::ZeroIsNotAllowed);
        }

        return Checked::Ok(a + b);
    }
}

impl std::ops::Neg for Checked<i32> {
    type Output = Checked<i32>;

    fn neg(self) -> Checked<i32> {
        let value = self?;

        if value == 0 {
            return Checked::Err(CheckedError::ZeroIsNotAllowed);
        }

        return Checked::Ok(-value);
    }
}
