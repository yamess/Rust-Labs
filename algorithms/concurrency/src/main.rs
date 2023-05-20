use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn my_thread_func<T>(n: i32, v: &mut Rc<Vec<i32>>) {
    let v_copy = v.clone();
    println!("Hello from my_thread_func at {}", n);
    thread::sleep(Duration::from_millis(1));
}

fn main() {
    let mut v= Rc::new(vec![1, 2, 3, 4, 5, 6, 7]);

    let my_func = |n:i32| {
        println!("Hello from my_func at {}", n);
    };
    v.iter().map(
        thread::spawn(|n| my_func(n)));

    println!("Hello, world!");
}
