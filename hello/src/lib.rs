use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数据量
    ///
    /// # Panics
    /// 
    /// `new` 函数在 size 为 0 时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool {
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        assert!(id > 0);
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}