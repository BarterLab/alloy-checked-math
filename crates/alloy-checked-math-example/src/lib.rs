#![feature(stmt_expr_attributes)]

pub mod models;
pub mod non_models;

#[cfg(test)]
#[test]
fn check_checked() {
    alloy_checked_math_lint::assert_checked(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/models").as_path());
}
