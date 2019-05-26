use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct Worker{
    id: u32,
    thread: thread::JoinHandle<()>
}

impl Worker{
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{

        let thread = thread::spawn(move || {
            loop{
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got job, executing...", id);
                job.call_box();
            }
        });

        Worker{
            id,
            thread
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool{
    ///Creates a new thread pool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let(sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size{
            //Create some threads here and store them in the vector
            workers.push(Worker::new(i as u32, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender
        }
    }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
    }
}
