use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use crate::job::{Message};
use crate::worker::Worker;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Message>();
        // The Arc type will let multiple workers own the receiver, and the Mutex will ensure that only one worker gets a Job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // The call to Worker::new creates the threads and sends them the receiving end of the channel.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender, }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // The loop will send a terminate message to all the workers, and then it will wait for each worker to finish.
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            // The call to worker.thread.join blocks the current thread until the worker’s thread finishes.
            // This will happen after the worker has executed the Job it was given.
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}