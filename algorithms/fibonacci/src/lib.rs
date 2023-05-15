
/// Calculates the nth Fibonacci number using the simple recursive approach.
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0  => 1,
        1  => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Calculates the nth Fibonacci number using the dynamic programming approach.
pub fn fibonacci_dp(n: u32) -> u32 {
    let mut previous = 1;
    let mut second_previous = 1;
    let mut result = 1;

    for _ in 1..n {
        result = previous + second_previous;
        second_previous = previous;
        previous = result;
    }
    return result;
}