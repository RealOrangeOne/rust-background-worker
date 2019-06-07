extern crate executors;

use executors::Executor;

use executors::crossbeam_workstealing_pool::ThreadPool;
use std::{thread, time};

struct Worker {
    pool: ThreadPool,
}

trait Workable: Send + Clone + 'static {
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

#[derive(Clone)]
struct Job1 {
    data: [i64; 1000],
}

impl Workable for Job1 {
    fn execute(&self) {
        println!("Executed job on {:?}", thread::current().name());
    }
}

fn main() {
    let worker = Worker::start(num_cpus::get());
    worker.schedule(Job1 {
        data: [2000000000; 1000],
    }); // Big data!
    thread::sleep(time::Duration::from_millis(1000));
    println!("Done");
}
