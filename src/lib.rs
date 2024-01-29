use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>, // The different threads that procces the http requests
    sender: Option<mpsc::Sender<Box<dyn FnOnce() -> Result<(),()> + Send + 'static>>>, // The communication channel to distribute to http requests
}

type Job = Box<dyn FnOnce() -> Result<(),()> + Send + 'static>;

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

        // Create a communiction channel from the main loop to the threads
        let (sender, receiver) = mpsc::channel();

        // Makes it so the communication channel can be shared between an arbitrary amount of threads
        let receiver = Arc::new(Mutex::new(receiver));

        // Create the vector to conatain all the worker threads
        let mut workers = Vec::with_capacity(size);

        // Creates all the worker threads, gives them an id and the reciver part of the communication channel
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // Executes a function with the correct signature. Used the handle one http request
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> Result<(),() > + Send + 'static,
    {
        // Put the box in a container to be sent to a thread
        let job = Box::new(f);

        // Sends the function to the thread that has the mutex lock
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl<'a> Drop for ThreadPool {
    fn drop(&mut self) {
        // Drops the communication channel which also gives the workers Err when trying to receive a message  
        drop(self.sender.take());

        // Collects all the workers and joins the threads 
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// Takes an id and the reciving end of the communction channel to the main thread. 
// Starts a thread that loops forever or until it gets erroneous data.

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop { 
            // Acquire the mutex if it's already taken by another thread it waits 
            // Then waits for something to be sent through the communication channel
            let message = receiver.lock().unwrap().recv(); 

            match message {
                // If the message is Ok execute the function 
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    // If the job returns Err print the error (right now only prints that it is an error)
                    if job().is_err() {
                        println!("Worker {id} got an error while proccesing")
                    };
                }
                // If the message is erroneous shut down the thread
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

