extern crate executors;

use executors::Executor;

use executors::crossbeam_workstealing_pool::ThreadPool;
use std::{thread, time};

struct Worker {
    pool: ThreadPool,
}

trait Workable: Send + 'static {
    fn execute(&self);
}

impl Worker {
    pub fn start(threads: usize) -> Self {
        return Worker {
            pool: ThreadPool::new(threads),
        };
    }

    pub fn schedule(&self, job: impl Workable) {
        self.pool.execute(move || {
            println!("Executing job");
            job.execute();
            println!("Executed job");
        });
    }
}

struct Job1;

impl Workable for Job1 {
    fn execute(&self) {
        println!("Executed job on {:?}", thread::current().name());
    }
}

fn main() {
    let worker = Worker::start(num_cpus::get());
    worker.schedule(Job1 {});
    thread::sleep(time::Duration::from_millis(1000));
    println!("Done");
}
