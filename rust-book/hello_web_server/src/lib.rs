use core::fmt;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug, Clone)]
pub struct PoolCreationError;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadWorker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<ThreadWorker>,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to create the ThreadPool")
    }
}

impl ThreadWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> ThreadWorker {
        let builder = thread::Builder::new();
        let handle = builder
            .spawn(move || loop {
                let msg = receiver.lock().expect("Retrieving lock failed").recv();
                match msg {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down");
                        break;
                    }
                }
            })
            .expect("Error creating thread for job");
        ThreadWorker {
            id,
            handle: Some(handle),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool::init_pool(size)
    }

    /// Build a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `new` function will return an error of PoolCreationError if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        Ok(ThreadPool::init_pool(size))
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    fn init_pool(size: usize) -> ThreadPool {
        let (sender, r) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(r));
        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(ThreadWorker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap()
            }
        }
    }
}
