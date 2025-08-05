fn id(x: i32) -> i32 {
    x
}

pub fn example(y: i32) -> i32 {
    let x = 1 + 1;
    let z = 1 + x + -id(y + y);
    return z;
}

#[cfg(test)]
#[test]
fn example_test() {
    assert_eq!(example(3), -3);
    assert_eq!(example(0), 3);
}
