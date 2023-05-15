use fibonacci::{fibonacci, fibonacci_dp};
use std::env;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let n = match args[0].parse::<u32>() {
        Ok(v) => v,
        Err(e) => panic!("Invalid number!!!. Error {}", e),
    };

    println!("==================== Fibonacci - Simple recursive approach ====================");
    println!("fibonacci({}) = {}", n, fibonacci(n));
    println!("==================== Fibonacci - Dynamic programming approach ====================");
    println!("fibonacci_dp({}) = {}", n, fibonacci_dp(n));
}
