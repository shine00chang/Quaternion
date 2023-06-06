use parking_lot::{RwLock, Mutex, Condvar};
use super::tree::*;
use super::game;


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
            node_limit: 1,
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

    pub fn solution (&self) -> Result<(Node, game::State), ()> {
        self.tree.read().solution()
    }

    pub fn advance (&self, state: &game::State) {
        self.stop(&mut self.state.lock());
        self.tree.write().advance(state);
        self.state.lock().nodes = 0;
        self.blocker.notify_all();
        self.start(&mut self.state.lock());
    }

    fn work (&self) {
        let (leaf, state) = {
            let tree = self.tree.read();
            if let Some (out) = tree.select() {
                out
            } else {
                return;
            }
        };
                
        // If too deep
        if state.pieces.is_empty() {
            return;
        }
        
        let children = gen_children(&state);

        { // Add children
            { 
                self.state.lock().nodes += children.len() as u64;
            }
            let mut leaf = leaf.lock(); 
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
