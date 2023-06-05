mod tree;
mod worker;


use std::thread;
use std::sync::Arc;
use std::time::{Duration, Instant};


fn driver (threads: u64, iters: u64, wait_ms: u64) {

        // Create worker.
        let worker = Arc::new(worker::Worker::new());

        // Spawn in worker threads.
        let _handles: Vec<_> = 
            (0..threads)
            .map(|_| {
                let worker = worker.clone();
                thread::spawn(move || { worker.work_loop() })
            })
            .collect(); 

        worker.start(&mut worker.state.lock());
        for _ in 0..iters {
            thread::sleep(Duration::from_millis(wait_ms));
            let res = worker.advance().expect("worker.advance() returned Err");
            println!("result: {res}");
        }
    }


fn main () {
    driver(10, 100, 333);
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::time::{Instant, Duration};
    use std::sync::Arc;
    use std::thread::{self, JoinHandle};
    use crate::worker::Worker;

    fn bench_by_nodes (threads: u64, nodes: u64) {
        // Create worker.
        let worker = Arc::new(Worker::new());
        {
            let mut worker_state = worker.state.lock();
            worker_state.node_limit = nodes;
        }

        // Spawn in worker threads.
        let _handles: Vec<JoinHandle<_>> = 
            (0..threads)
            .map(|_| {
                let worker = worker.clone();
                thread::spawn(move || { worker.work_loop() })
            })
            .collect(); 

        // Wait for node generation.
        let start = Instant::now();
        worker.start(&mut worker.state.lock());
        thread::sleep(Duration::from_millis(100));
        worker.stop(&mut worker.state.lock());

        // Log result.
        {
            let worker_state = worker.state.lock();
            let elapsed_ms = Instant::now().duration_since(start).as_millis();

            print!("{}ms:{}", elapsed_ms, worker_state.nodes);
        }
    }

    #[test]
    fn thread_test () {
        let threads_tests = [8, 10, 12, 14, 16];
        let nodes_tests = [1000, 10000, 100000, 1000000];

        print!("Threads: \t");
        for i in threads_tests { print!("{}\t\t", i); }
        println!();

        for nodes in nodes_tests {
            print!("{}\t\t", nodes);
            for threads in threads_tests {
                bench_by_nodes(threads, nodes);
                print!("\t");
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
    }
    
    #[test]
    fn advance_test () {
        let threads = 10;
        let iters   = 10;
        let wait_ms = 100;

        // Create worker.
        let worker = Arc::new(Worker::new());

        // Spawn in worker threads.
        let _handles: Vec<JoinHandle<_>> = 
            (0..threads)
            .map(|_| {
                let worker = worker.clone();
                thread::spawn(move || { worker.work_loop() })
            })
            .collect(); 

        worker.start(&mut worker.state.lock());
        for _ in 0..iters {
            thread::sleep(Duration::from_millis(wait_ms));
            let res = worker.advance().expect("worker.advance() returned Err");
            println!("result: {res}");
        }
    }
}
