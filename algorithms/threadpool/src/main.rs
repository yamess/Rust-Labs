mod job;
mod worker;
mod threadpool;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(10);
    for i in 0..10000 {
        pool.execute(move || {
            println!("Task {} is being executed", i);
        });
    }
}
