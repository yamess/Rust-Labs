use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use crate::job::{Job, Message};

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || loop {
            // The loop will keep the thread alive for the lifetime of the program.
            // The call to recv blocks, so the thread will wait until a Job becomes available.
            // Once a Job is received, the thread will execute the Job by calling its execute method.
            // The loop will then go back to waiting for a Job.
            let message = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                },
            }
        }));
        Worker { id, thread }
    }
}