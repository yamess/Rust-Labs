mod job;
mod worker;
mod threadpool;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(20);
    for i in 0..100 {
        pool.execute(move || {
            println!("Task {} is being executed", i);
        });
    }
}
