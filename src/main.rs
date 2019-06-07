extern crate executors;

use executors::Executor;

use executors::crossbeam_workstealing_pool::ThreadPool;


struct Worker {
    pool: ThreadPool
}

trait Workable: Send {
    fn execute(&self);
}


impl Worker {
    pub fn start() -> Self {
        return Worker {
            pool: ThreadPool::new(4)
        }
    }

    pub fn schedule<T: Workable>(&self, job: T) {
        self.pool.execute(|| {
            job.execute();
        });
    }
}

struct Job1;

impl Workable for Job1 {
    fn execute(&self) {
        println!("Executed");
    }
}


fn main() {
    let worker = Worker::start();
    worker.schedule(Job1 {});
}
