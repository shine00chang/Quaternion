mod game;
mod tree;
mod worker;

// Re-Exports (for driver)
pub use game::{Piece, Key, State, Move};
pub use worker::BotStats;
pub use game::MoveStats;

// For single-threaded WASM driver
pub use worker::Worker;

#[cfg(not(target_family = "wasm"))]
pub use game::sim::SimState;

use std::sync::Arc;

#[cfg(not(target_family = "wasm"))]
use std::thread;



pub struct Quaternion {
    // public for WASM worker. Needs access to it to implement the single-thread workaround.
    worker: Arc<Worker>,
    #[cfg(not(target_family = "wasm"))]
    handles: Vec<thread::JoinHandle<()>>
}


impl Quaternion {
    pub fn start (&self) { 
        self.worker.start(&mut self.worker.state.lock());
    }

    pub fn stop (&self) {
        self.worker.stop(&mut self.worker.state.lock());
    }

    pub fn solution (&self) -> Move {
        self.worker
            .solution()
            .expect("worker.solution() returned Err")
            .mv
    }

    /// Exposed interface for bot advancement.
    /// Takes in a `SimState`, converts it into a `State`, and passes it to the worker.
    pub fn advance (&self, state: &State) {
        self.worker.advance(&state);
    }

    pub fn stats (&self) -> BotStats { 
        self.worker.state.lock().stats.clone()
    }
}

#[cfg(not(target_family = "wasm"))]
impl Quaternion {
    #[cfg(not(target_family = "wasm"))]
    pub fn with_threads(threads: u32) -> Self {
        // Spawn in worker threads.
        let worker = Arc::new(worker::Worker::new());
        
        let handles: Vec<_> = 
            (0..threads)
            .map(|_| {
                let worker = worker.clone();
                thread::spawn(move || { worker.work_loop() })
            })
            .collect(); 
        Self {
            worker,
            handles
        }
    }

    pub fn end (self) {
        self.stop();
        self.handles.into_iter().for_each(|handle| handle.join().expect("failed to join worker"));
    }
}


#[cfg(target_family = "wasm")]
use std::time::Duration;
#[cfg(target_family = "wasm")]
impl Quaternion {
    // For WASM driver. Does not spawn threads
    pub fn single () -> Self {
        let worker = Arc::new(worker::Worker::new());
        
        Self { worker }
    }

    // For WASM driver. A single-threaded version.
    pub fn wasm_run (&self, delay_ms: u32) -> Move {

        // Get timestamp
        /*
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let start = performance.now();
        */

        self.start();
        //while start < delay_ms as f64 {
        for _ in 0..1000 {
            self.worker.work();
        }
        self.stop();
        self.solution()
    }
}
