use super::*;

#[test]
fn test_fib_loop() {
    assert_eq!(fibonacci_loop(0), 0);
    assert_eq!(fibonacci_loop(10), 55);
}

#[test]
fn test_fib_recursive() {
    assert_eq!(fibonacci_recursive(1), 1);
    assert_eq!(fibonacci_recursive(7), 13);
}

#[test]
fn test_temps() {
    assert_eq!(celsius2fahrenheit(0), 32);
    assert_eq!(fahrenheit2celsius(32), 0);
}
