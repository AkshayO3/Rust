use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct Threadpool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
impl Threadpool {
    pub fn new(num_threads:usize) -> Threadpool {
        assert!(num_threads > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(num_threads);
        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        Threadpool{workers, sender}
    }
    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>,
}
impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Worker{id, thread}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;