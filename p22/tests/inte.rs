use p22::calc::fibonacci_recursive;

#[test]
fn test_fib_recursive() {
    assert_eq!(fibonacci_recursive(1), 1);
    assert_eq!(fibonacci_recursive(7), 13);
}
