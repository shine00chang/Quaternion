/*
two threads expanding a tree at the same time.
thread routine: 
- Select Node
	- Get an explored children
	- If all explored, pick first.
- Generate children
- Append children
- Stop after certain layer reached

Testing Protocol:
- Expected result: 
	- Binary Tree with one side explored to level N 

- Execute with one thread, mark time.
- Execute with multiple threads, mark time.
*/

use rand::prelude::*;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::sync::Arc;
use parking_lot::{
    Mutex,
    RwLock,
};


#[derive(Debug)]
struct A {
    n: i32,
    v: Vec<Arc<Mutex<A>>>,
    expanding: bool
}
impl Default for A {
    fn default() -> Self {
        Self { 
            n: 0,
            v: vec![],
            expanding: false
        }
    }
}
struct B {
    a: Arc<Mutex<A>>
}
enum SelectionResult {
    Continue (Arc<Mutex<A>>),
    Deadend,
    Leaf,
}
impl A {
    pub fn select (&mut self) -> SelectionResult {
        if self.expanding {
            SelectionResult::Deadend
        } else 
        if self.v.is_empty() {
            SelectionResult::Leaf
        } else {
            let candidates: Vec<_> = self.v.iter().filter(|child| !child.lock().expanding).collect();
            if candidates.is_empty() { return SelectionResult::Deadend }

            let mut rng = rand::thread_rng();
            let i: usize = (rng.gen::<f64>() * candidates.len() as f64) as usize;
            SelectionResult::Continue(candidates[i].clone())
        }
    }
}

static DEADEND_COUNTER: RwLock<usize> = RwLock::new(0);
static NODE_COUNTER: RwLock<usize> = RwLock::new(0);
const SLEEP_MS: u64 = 10;

fn routine (arc_root: &Arc<RwLock<Box<B>>>) {
    let mutex_leaf: Arc<Mutex<A>> = {
        let mut mutex_node: Arc<Mutex<A>> = {
            let root = arc_root.read();
            root.a.clone()
        };

        loop {
            let mut node = mutex_node.lock();
            match node.select() {
                SelectionResult::Continue(mutex_child)  => { drop(node); mutex_node = mutex_child },
                SelectionResult::Leaf                   => { node.expanding = true; break; }
                SelectionResult::Deadend                => { 
                    drop(node);
                    thread::sleep(Duration::from_millis(10));
                    {
                        *DEADEND_COUNTER.write() += 1;
                    }
                    continue; 
                },
            }
        }
        mutex_node
    };
    
    let children: Vec<Arc<Mutex<A>>> = { // Gen children
        let mut rng = rand::thread_rng();
        let mut children = vec![];
        for _ in 0..10 {
            children.push( Arc::new(Mutex::new( A{
                n: (rng.gen::<f64>() * 100.0) as i32,
                v: vec![],
                expanding: false
            })));
        }
        thread::sleep(Duration::from_millis(SLEEP_MS));
        children
    };

    { // Add children
        { 
            *NODE_COUNTER.write() += children.len();
        }
        let mut leaf = mutex_leaf.lock(); 
        leaf.v = children;
        leaf.expanding = false;
    }
}

fn test (threads: usize, nodes: usize) {
    let start = Instant::now();

    let root = Box::new(B { a: Default::default() });

    let arc_rw_root = Arc::new(RwLock::new(root));
    let mut handles = vec![];

    for _ in 0..threads {
        let arc_rw_root = arc_rw_root.clone();
        handles.push( thread::spawn(move || {
            while *NODE_COUNTER.read() < nodes {
                routine(&arc_rw_root);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed_ms = Instant::now().duration_since(start).as_millis();
    print!("{}ms", elapsed_ms);

    // Print
    /*
    {
        let root = arc_rw_root.read().unwrap();
        println!("Root");
        let mut queue: Vec<Arc<Mutex<A>>> = vec![root.a.clone()];
        let mut cnt = 0;
        while !queue.is_empty() {
            let mut next: Vec<Arc<Mutex<A>>> = vec![];
            for mutex_node in queue {
                let node = mutex_node.lock().unwrap();

                print!("[");
                for node in &node.v {
                    print!("{}\t", node.lock().unwrap().n);
                }
                print!("]");

                if !node.v.is_empty() { cnt += 1; }
                next.append( &mut node.v.iter().map(|child: &Arc<Mutex<A>>| child.clone()).collect() );
            }
            Wprintln!();
            queue = next;
        }
        println!("\nbranch nodes: {}", cnt);
    }
    */
}

fn main () {
    let threads_tests = [1, 2, 4, 6, 8, 10, 12];
    let nodes_tests = [1000, 10000, 100000];

    print!("Threads: \t");
    for i in threads_tests { print!("{}\t\t", i); }
    println!();

    for nodes in nodes_tests {
        print!("{}\t", nodes);
        for threads in threads_tests {
            test(threads, nodes);
            print!(":{}:{}\t", *NODE_COUNTER.read()-nodes, DEADEND_COUNTER.read());
            std::io::stdout().flush().unwrap();
            *NODE_COUNTER.write() = 0;
            *DEADEND_COUNTER.write() = 0;
        }
        println!();
    }
}
