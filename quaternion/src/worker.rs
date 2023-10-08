use std::sync::Arc;

use parking_lot::{RwLock, Mutex, Condvar};
use super::tree::*;
use super::game;


pub struct Worker {
    tree: RwLock<Tree>,
    pub state: Mutex<State>,
    pub blocker: Condvar
}

#[derive(Clone)]
pub struct State {
    pub node_limit: u64,
    pub run: bool,
    pub stats: BotStats,
}

#[derive(Clone, Default)]
pub struct BotStats {
    pub nodes: u64
}

impl Default for State {
    fn default() -> Self {
        Self {
            stats: Default::default(),
            node_limit: 100000,
            run: false
        }
    }
}

impl State {
    fn should_work(&self) -> bool {
        self.run && self.stats.nodes < self.node_limit 
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

    /// Starts worker cycle. Panics if is already running
    pub fn start (&self, state: &mut State) {
        assert!(state.run == false);
        state.run = true;
        self.blocker.notify_all();
    }

    /// Starts worker cycle. Panics if is not running 
    pub fn stop (&self, state: &mut State) {
        assert!(state.run == true);
        state.run = false;
        self.blocker.notify_all();
    }

    pub fn solution (&self) -> Result<Node, ()> {
        self.tree.read().solution()
    }

    /// Advance worker into new state.
    /// Does not affect running/stopping state of the bot.
    pub fn advance (&self, state: &game::State) {
        // If is running, stop.
        let was_running = {
            let state = &mut self.state.lock();
            if state.run {
                self.stop(state);
                true
            } else { false }
        };

        self.tree.write().advance(state);

        // Reset Stats
        self.state.lock().stats = Default::default();

        // If was running, continue.
        if was_running {
            self.start(&mut self.state.lock());
        }
    }

    fn work (&self) {
        let selection = 
            if let Some (out) = self.tree.read().select() {
                out
            } else {
                return
            };
            
        // If too deep
        if selection.get_state().queue_len() <= 2 {
            return
        }
        
        let children: Vec<_> = {
            let nodes = gen_children(selection.get_state());
            self.state.lock().stats.nodes += nodes.len() as u64;

            let nodes = prune_children(nodes, &selection);
            nodes
                .into_iter()
                .map(|i| Arc::new(Mutex::new(i)))
                .collect()
        };

        { // Add children
            let backprop = selection.expand(children);
            selection.backprop(backprop);
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
