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

struct SharedData {
    queue: SafeQueue<Job>,
    queued_count: AtomicUsize,
    active_count: AtomicUsize,
    wait_mutex: Mutex<()>,
    wait_cond: Condvar,
    shutdown: AtomicBool,
    
}

impl SharedData {
    fn new() -> SharedData {
        SharedData {
            queue: SafeQueue::new(),
            queued_count: 0.into(),
            active_count: 0.into(),
            wait_mutex: Mutex::new(()),
            wait_cond: Condvar::new(),
            shutdown: false.into(),
        }
    }

    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0 ||
            self.active_count.load(Ordering::SeqCst) > 0
    }

}

type Job = Box<dyn FnOnce() + Send + 'static>;
type WorkerHandle = JoinHandle<()>;
struct Worker {
    shared: Arc<SharedData>,
}

impl Worker {
    fn new(shared: Arc<SharedData>) -> Worker {
        Worker { shared }
    }
    fn run(&self) -> WorkerHandle{
        let shared = self.shared.clone();

        thread::spawn(move ||{
            while !shared.shutdown.load(Ordering::Relaxed) {
                let wait_guard = shared.wait_mutex.lock().unwrap();
                if shared.queue.empty() {
                    shared.wait_cond.wait(wait_guard).unwrap();
                }
                let job = shared.queue.dequeue();
                std::mem::drop(wait_guard);
                shared.active_count.fetch_add(1, Ordering::SeqCst);
                shared.queued_count.fetch_sub(1, Ordering::SeqCst);
                job();
                shared.active_count.fetch_sub(1, Ordering::SeqCst);
            }
        })
    }
}

struct ThreadPool {
    shared: Arc<SharedData>,
    workers: Vec<Worker>,
    handlers: Vec<WorkerHandle>,
}

impl ThreadPool {
    fn new(thread_num: usize) -> ThreadPool {
        
        let mut workers = Vec::with_capacity(thread_num);
        let shared = Arc::new(SharedData::new());

        for _ in 0..thread_num {
            workers.push(Worker::new(shared.clone()));
        }

        let handlers = Vec::with_capacity(thread_num);
        let working_number = AtomicUsize::new(0);

        ThreadPool {shared, workers, handlers}
    }
    fn start(&mut self) {
        for worker in &self.workers {
            self.handlers.push(worker.run());
        }
    }

    fn shutdown(self) {
        self.shared.shutdown.store(true, Ordering::Relaxed);
        self.shared.wait_cond.notify_all();

        for handler in self.handlers {
            handler.join().unwrap();
        }
        
    }

    fn join(&self) {
        
    }

    fn submit(&self, task: Job) {
        self.shared.queue.enqueue(task);
        self.shared.wait_cond.notify_one();
    }
}

fn main() {
    let mut thread_pool = ThreadPool::new(4);
    thread_pool.submit(Box::new(||{println!("Hello world!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.start();
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Hello Biden!")}));
    thread_pool.submit(Box::new(||{println!("Goodbye Putin!")}));
    thread::sleep(Duration::from_secs(10));
}