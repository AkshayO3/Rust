use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct Threadpool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl Threadpool {
    pub fn new(num_threads:usize) -> Threadpool {
        assert!(num_threads > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}
impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message{
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                },
            }
        });
        Worker{id, thread:Some(thread)}
    }
}

impl Drop for Threadpool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

enum Message{
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;