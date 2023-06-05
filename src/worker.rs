use std::sync::Arc;
use parking_lot::{RwLock, Mutex, Condvar};
use super::tree::*;

pub struct Worker {
    tree: RwLock<Tree>,
    pub state: Mutex<State>,
    pub blocker: Condvar
}

pub struct State {
    pub nodes: u64,
    pub node_limit: u64,
    pub run: bool
}

impl Default for State {
    fn default() -> Self {
        Self {
            nodes: 0,
            node_limit: 100000,
            run: false
        }
    }
}

impl State {
    fn should_work(&self) -> bool {
        self.run && self.nodes < self.node_limit 
    }
}

impl Worker {
    pub fn new () -> Self {
        Self {
            tree: Default::default(),
            state: Default::default(),
            blocker: Condvar::new()
        }
    }

    pub fn start (&self, state: &mut State) {
        state.run = true;
        self.blocker.notify_all();
    }

    pub fn stop (&self, state: &mut State) {
        state.run = false;
        self.blocker.notify_all();
    }

    pub fn advance (&self) -> Result<i32, ()> {
        let out = self.tree.write().advance();
        self.state.lock().nodes = 0;
        self.blocker.notify_all();
        out
    }

    fn work (&self) {
        let mutex_leaf: Arc<Mutex<Node>> = {
            let tree = self.tree.read();
            tree.select()
        };
                
        let children = {
            let leaf = mutex_leaf.lock().clone();
            gen_children(&leaf)
        };

        { // Add children
            { 
                self.state.lock().nodes += children.len() as u64;
            }
            let mut leaf = mutex_leaf.lock(); 
            leaf.expand(children);
        }
    }

    pub fn work_loop (&self) {
        loop {
            // Check if should work. If not, wait on Condvar.
            {
                let mut state = self.state.lock();
                while !state.should_work() {
                    self.blocker.wait(&mut state);
                }
            }
            self.work();
        }
    }
}
