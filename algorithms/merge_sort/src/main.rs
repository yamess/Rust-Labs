use merge_sort::merge_sort;
use rand::{Rng, thread_rng};
use std::env::args;
use std::iter::repeat_with;
use std::process;


fn main() {
   let args: Vec<String> = args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: merge_sort <number of elements>");
        process::exit(1);
    }
    let n: i64 = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Usage: merge_sort <number of elements>");
            process::exit(1);
        }
    };
    let mut rng = thread_rng();
    let mut my_vec: Vec<i64> = repeat_with(|| rng.gen_range(0..n))
        .take(n as usize)
        .collect();
    println!("Initial vector: {:?}", my_vec);
    my_vec = merge_sort(my_vec);
    println!("Sorted vector: {:?}", my_vec);

}
