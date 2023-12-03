#[test]
fn my_test() {
    let value = 21 - 2;
    assert_eq!(value, 19);
}

#[test]
fn not_equals() {
    assert_ne!(2, 1);
}

#[test]
fn assert_bool() {
    let mut result = false;
    if 22 > 3 {
        result = true;
    }

    // is assert if bool is true else thorw error
    assert!(result);
}
