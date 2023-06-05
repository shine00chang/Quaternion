mod game;
mod tree;
mod worker;


use std::thread;
use std::sync::Arc;
use std::time::{Duration, Instant};

use rand::prelude::*;

pub fn draw (bag: &mut Vec<game::Piece>) -> game::Piece {
    if bag.is_empty() {
        bag.push(game::Piece::J);
        bag.push(game::Piece::L);
        bag.push(game::Piece::S);
        bag.push(game::Piece::Z);
        bag.push(game::Piece::T);
        bag.push(game::Piece::I);
        bag.push(game::Piece::O);
    }
    let mut rng = rand::thread_rng();
    let i = rng.gen::<f64>() * bag.len() as f64;
    let p = bag.remove(i as usize);
    p
}

fn driver (threads: u64, iters: u64, wait_ms: u64) {
    // Create worker.
    let worker = Arc::new(worker::Worker::new());
    let mut bag = vec![];

    // Create initial game state.
    {
        let mut state = tetron::State::new(); 
        while state.pieces.len() < 5 {
            state.pieces.push_back(draw(&mut bag));
        }
        worker.advance(&state); 
    }

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
        let (_, mut state) = worker.solution().expect("worker.solution() returned Err");

        while state.pieces.len() < 5 {
            state.pieces.push_back(draw(&mut bag));
        }

        worker.advance(&state);
        println!("{}\n", state);

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
}
