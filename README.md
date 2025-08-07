```rust
use alloy_checked_math_core::CheckedMathError;
use alloy_checked_math_macro::{checked, unchecked};

fn safe_example(x: i32) -> Result<i32, CheckedMathError> {
    let x = checked! { 1 / x };
    return Ok(x);
}

fn explicit_unsafe_example(x: i32) -> i32 {
    let x = unchecked! { 1 / x };
    return x;
}

fn implicit_unsafe_example(x: i32) -> i32 {
    let x = 1 / x; // warning will be here
    return x;
}

#[test]
fn test_all_math_is_checked() {
    alloy_checked_math_lint::assert_checked(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
    );
}
```
