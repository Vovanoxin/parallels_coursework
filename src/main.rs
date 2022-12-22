use threadpool::ThreadPool;
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let pool = ThreadPool::new(8);
    let at = Arc::new(AtomicUsize::new(0));
    for _ in 0..100 {
        let at = at.clone();
        pool.execute(move || {
            at.fetch_add(1, Ordering::Relaxed);
        })
    }
    pool.join();
    println!("{}", at.load(Ordering::Relaxed));
}