// Docs: https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html#building-the-threadpool-struct-using-compiler-driven-development

// Building the ThreadPool Struct Using Compiler Driven Development

pub struct ThreadPool;

impl ThreadPool {
    // usize, because negative number of threads doesn't make sense
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    // We still use the () after FnOnce because this FnONce represents a closure that takes not
    // parameters and returns the unit type ()
    // The F type pararmeter slo has the trait bound Send and the lifetime bound 'static, which are
    // useful in our situation: we need Send to tranfer the closure form one thread to another and
    // 'static because we don't know how long the thread will takte to execute
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
