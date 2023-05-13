use bubble_sort::bubble_sort;
use std::iter::repeat_with;
use rand::Rng;
use rand::thread_rng;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Please enter a valid number");
        process::exit(1);
    }
    let n: i64 = match args[0].parse() {
        Ok(v) => v,
        Err(e) => panic!("Invalid number!!!. Error {}", e),
    };

    let mut rng = thread_rng();
    let mut my_vec: Vec<i64> = repeat_with(|| rng.gen_range(0..n))
        .take(n as usize)
        .collect();
    println!("Initial vector: {:?}", my_vec);
    bubble_sort(&mut my_vec);
    println!("Sorted vector: {:?}", my_vec);
}
