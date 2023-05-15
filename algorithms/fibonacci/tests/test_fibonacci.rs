use fibonacci::{fibonacci, fibonacci_dp};

/// Calculates the nth Fibonacci number using the simple recursive approach.
/// Calculates the nth Fibonacci number using the dynamic programming approach.
#[test]
fn test_fibonacci(){
    let a = 0 as u32;
    let b = 1_u32;
    let n: u32 = 10;

    assert_eq!(fibonacci(a), 1);
    assert_eq!(fibonacci(b), 1);
    assert_eq!(fibonacci(n), 89);

    assert_eq!(fibonacci_dp(a), 1);
    assert_eq!(fibonacci_dp(b), 1);
    assert_eq!(fibonacci_dp(n), 89);
}