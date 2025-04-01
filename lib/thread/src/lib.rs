use std::thread;
pub struct ThreadManager {
    threads: Vec<thread::JoinHandle<()>>,
    thread_usize: usize,
}

impl ThreadManager {
    pub fn new(thread_usize: usize) -> Self {
        ThreadManager {
            threads: Vec::new(),
            thread_usize,
        }
    }

    pub fn add(&mut self, th: thread::JoinHandle<()>) {
        println!("new thread");
        self.threads.push(th);
        self.check();
    }
    pub fn wait(&mut self) {
        while let Some(th) = self.threads.pop() {
            th.join().unwrap();
            println!("delete thread");
        }
    }
    pub fn check(&mut self) {
        if self.threads.len() > self.thread_usize {
            self.wait();
        }
    }
}
