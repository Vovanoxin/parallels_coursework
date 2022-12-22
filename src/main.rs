use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::{Mutex, Arc, Condvar};
use std::collections::VecDeque;
use std::thread::{JoinHandle, self};
use std::time::Duration;

struct SafeQueue<T> {
    dequeue: Mutex<VecDeque<T>>,
}
impl<T> SafeQueue<T> {
    fn new() -> SafeQueue<T> {
        let dequeue = Mutex::new(VecDeque::new());
        SafeQueue { dequeue }
    }

    fn enqueue(&self, v: T) {
        let mut guard = self.dequeue.lock().unwrap();
        guard.push_back(v);   
    }

    fn dequeue(&self) -> T {
        let mut guard = self.dequeue.lock().unwrap();
        guard.pop_front().unwrap()
    }

    fn size(&self) -> usize {
        let guard = self.dequeue.lock().unwrap();
        guard.len()
    }

    fn empty(&self) -> bool {
        self.size() == 0
    }
    
}

struct WorkerControl {
    wait_mutex: Mutex<()>,
    wait_cond: Condvar,
    shutdown: AtomicBool,
}

impl WorkerControl {
    fn new() -> WorkerControl {
        WorkerControl { wait_mutex: Mutex::new(()), wait_cond: Condvar::new(), shutdown: false.into() }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type WorkerHandle = JoinHandle<()>;
struct Worker {
    control: Arc<WorkerControl>,
    task_queue: Arc<SafeQueue<Job>>,
}

impl Worker {
    fn new(control: Arc<WorkerControl>, task_queue: Arc<SafeQueue<Job>>) -> Worker {
        Worker { control: control.clone(),  task_queue: task_queue.clone()}
    }
    fn run(&self) -> WorkerHandle{
        let control = self.control.clone();
        let queue = self.task_queue.clone();
        thread::spawn(move ||{
            while !control.shutdown.load(Ordering::Relaxed) {
                let wait_guard = control.wait_mutex.lock().unwrap();
                if queue.empty() {
                    control.wait_cond.wait(wait_guard).unwrap();
                }
                let job = queue.dequeue();
                job();
            }
        })
    }
}

struct ThreadPool {
    queue: Arc<SafeQueue<Job>>,
    working_number: AtomicUsize,
    control: Arc<WorkerControl>,
    workers: Vec<Worker>,
    handlers: Vec<WorkerHandle>,
}

impl ThreadPool {
    fn new(thread_num: usize) -> ThreadPool {
        
        let mut workers = Vec::with_capacity(thread_num);
        let control = Arc::new(WorkerControl::new());
        let queue = Arc::new(SafeQueue::new());

        for _ in 0..thread_num {
            workers.push(Worker::new(control.clone(), queue.clone()));
        }

        let handlers = Vec::with_capacity(thread_num);
        let working_number = AtomicUsize::new(0);

        ThreadPool { queue, working_number, control, workers, handlers}
    }
    fn start(&mut self) {
        for worker in &self.workers {
            self.handlers.push(worker.run());
        }
    }

    fn shutdown(self) {
        self.control.shutdown.store(true, Ordering::Relaxed);
        self.control.wait_cond.notify_all();

        for handler in self.handlers {
            handler.join().unwrap();
        }
        
    }

    fn submit(&self, task: Job) {
        self.queue.enqueue(task);
        self.control.wait_cond.notify_one();
    }
}

fn main() {
    let mut thread_pool = ThreadPool::new(4);
    thread_pool.submit(Box::new(||{println!("Hello world!")}));
    thread_pool.start();
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Goodbye Putin!")}));
    thread::sleep(Duration::from_secs(10));
}