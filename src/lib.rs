use std::thread::{JoinHandle, spawn};
use std::sync::{mpsc,Arc,Mutex};
pub struct ThreadPool {
    pool : Vec<Worker>,
    sender : mpsc::Sender<Message>,
}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size:usize) -> ThreadPool{

        assert!(size > 0);
        let mut pool = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            let new_worker = Worker::new(id,Arc::clone(&receiver));
            pool.push(new_worker);
        }

        ThreadPool{
            pool,
            sender,
        }

    }
    pub fn execute<F>(&self,f:F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending Terminate to all the Workers");
        for _ in &self.pool{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting Down all Workers");
        for worker in &mut self.pool{
            println!("Shutting Down Worker {}",worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id:usize,
    thread : Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job)=>{
                    println!("Worker {} got a job; executing",id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} is Terminating",id);
                    break;
                }
                
            }
        });
        Worker{
            id,
            thread : Some(thread),
        }
    }
}

enum Message{
    NewJob(Job),
    Terminate,
}