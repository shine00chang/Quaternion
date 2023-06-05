use rand::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::Arc; 
use parking_lot::Mutex;

#[derive(Default)]
pub struct Tree {
    root: Arc<Mutex<Node>>,
}

impl Tree {
    pub fn select (&self) -> Arc<Mutex<Node>> {
        let mut mutex_node: Arc<Mutex<Node>> = self.root.clone(); 

        loop {
            let mut node = mutex_node.lock();
            match node.select() {
                SelectionResult::Continue(mutex_child)  => { drop(node); mutex_node = mutex_child },
                SelectionResult::Leaf                   => { node.expanding = true; break; }
                SelectionResult::Deadend                => { 
                    drop(node);
                    thread::sleep(Duration::from_millis(1));
                    continue;
                },
            }
        }
        mutex_node
    }

    pub fn advance (&mut self) -> Result<i32, ()> {
        let (score, child) = {
            let root = self.root.lock();
            let mut best: Result<(_, _), ()> = Err(());
            for child in &root.v {
                let child_score = child.lock().n;
                if let Ok((score, _)) = best {
                    if score < child_score {
                        best = Ok((child_score, child));
                    }
                } else {
                    best = Ok((child_score, child));
                }
            }
            best.map(|b| (b.0, b.1.clone()))
        }?;

        self.root = child;
        Ok(score)
    }
}

enum SelectionResult {
    Continue (Arc<Mutex<Node>>),
    Deadend,
    Leaf,
}

#[derive(Debug, Clone)]
pub struct Node {
    n: i32,
    v: Vec<Arc<Mutex<Node>>>,
    expanding: bool
}

impl Default for Node {
    fn default() -> Self {
        Self { 
            n: 0,
            v: vec![],
            expanding: false
        }
    }
}

impl Node {
    fn select (&self) -> SelectionResult {
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

    pub fn expand (&mut self, children: Vec<Arc<Mutex<Node>>>) {
        self.v = children;
        self.expanding = false;
    }
}

pub fn gen_children (parent: &Node) -> Vec<Arc<Mutex<Node>>> {
    let mut children = vec![];
    let layer = parent.n / 100 + 1;
    let prefix = layer * 100 + parent.n % 10 * 10;

    for i in 0..10 {
        children.push( Arc::new(Mutex::new( Node{
            n: prefix + i,
            v: vec![],
            expanding: false
        })));
    }
    children
}
