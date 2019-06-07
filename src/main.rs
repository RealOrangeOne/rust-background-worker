extern crate executors;

use executors::Executor;

use executors::crossbeam_workstealing_pool::ThreadPool;
use std::sync::mpsc::channel;


struct Worker {
    pool: ThreadPool
}

trait Workable: Send + Sync {
    fn execute(&self);
}


impl Worker {
    pub fn start() -> Self {
        return Worker {
            pool: ThreadPool::new(4)
        }
    }

    pub fn schedule<T: Workable + ?Sized>(&self, job: &'static T) {
        self.pool.execute(move || {
            job.execute()
        });
    }
}


fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    println!("Hello, world!");
    let n_jobs = 8;

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }
}
