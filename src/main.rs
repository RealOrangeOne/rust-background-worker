extern crate executors;

use executors::Executor;

use executors::crossbeam_workstealing_pool::ThreadPool;
use std::thread;
use std::time::{Duration, Instant};

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
            let now = Instant::now();
            job.execute();
            let elapsed = now.elapsed();
            println!("Executed job in {}", elapsed.as_micros());
        });
    }

    pub fn stop(self) {
        self.pool.shutdown().expect("Failed to shutdown worker");
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
    thread::sleep(Duration::from_millis(1000));
    println!("Done");
    worker.stop();
}
