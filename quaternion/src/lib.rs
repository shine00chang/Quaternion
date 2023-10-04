mod game;
mod gen;
mod eval;
mod tree;
mod worker;


use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use std::thread;

#[cfg(target_family = "wasm")]
use wasm_thread as thread;

use worker::Worker;

pub struct Quaternion {
    worker: Arc<Worker>,
    handles: Vec<thread::JoinHandle<()>>
}

impl Quaternion {
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

    pub fn start (&self) { 
        self.worker.start(&mut self.worker.state.lock());
    }

    pub fn stop (&self) {
        self.worker.stop(&mut self.worker.state.lock());
    }

    pub fn end (self) {
        self.stop();
        self.handles.into_iter().for_each(|handle| handle.join().expect("failed to join worker"));
    }

    pub fn solution (&self) -> (game::Move, game::State) {
        let (node, state) = self.worker.solution().expect("worker.solution() returned Err");
        (node.get_mv().clone(), state)
    }

    pub fn advance (&self, state: game::State) {
        self.worker.advance(&state);
    }

    pub fn stats (&self) -> worker::State { 
        self.worker.state.lock().clone()
    }
}
