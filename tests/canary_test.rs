#[test]
fn should_equal() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn should_not_equal() {
    assert_ne!(1, 2);
}

#[test]
fn should_true() {
    assert!(true);
}

#[test]
#[should_panic(expected = "Should Panic")]
fn should_panic() {
    panic!("Should Panic!")
}