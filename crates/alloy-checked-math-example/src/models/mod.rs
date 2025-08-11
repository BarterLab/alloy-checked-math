pub mod example;

#[cfg(test)]
#[test]
fn test_all_math_is_checked() {
    alloy_checked_math::assert_checked_subtree!();
}
