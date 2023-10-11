use std::sync::Arc; 
use parking_lot::{Mutex, RwLock};
use super::game;
use rand::Rng;

const CUTOFF_F: f32 = 0.2;

pub struct Tree {
    root_state: RwLock<game::State>,
    root: Arc<Mutex<Node>>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            root_state: RwLock::new( game::State::default() ),
            root: Default::default()
        }
    }
}

impl Tree {
    pub fn select (&self) -> Option<Selection> {
        let mut list = vec![];
        let (mut mutex_node, mut state) = {
            let state = self.root_state.read();
            (self.root.clone(), state.clone())
        };

        loop {
            let mut node = mutex_node.lock();
            match node.select() {
                SelectionResult::Continue(mutex_child)  => {
                    drop(node);
                    let child = mutex_child.lock();
                    state = state.apply_move(child.get_mv());
                    drop(child);
                    list.push(mutex_node);
                    mutex_node = mutex_child;
                },
                SelectionResult::Leaf    => { node.expanding = true; break; }
                SelectionResult::Deadend => return None, 
            }
        }
        list.push(mutex_node);
        Some(Selection::new(list, state))
    }

    fn print_best (&self) {
        let (mut mutex_node, mut state) = {
            let state = self.root_state.read();
            (self.root.clone(), state.clone())
        };

        loop {
            let node = mutex_node.lock();

            let best = 
                if node.children.len() == 0 {
                    Err(())
                } else {
                    Ok(node.children
                        .iter()
                        .fold((Arc::new(Mutex::new(Node::default())), f32::MIN), |a, n| {
                            let eval = &n.lock().eval;
                            if eval.get() > a.1 { (n.clone(), eval.get()) } 
                            else { a }
                        })
                        .0)
                };

            if let Ok(next) = best {
                drop(node);
                let child = next.lock();
                let mut stats = Default::default();
                (state, stats) = state.apply_move_with_stats(child.get_mv());
                println!("{:?}\t{:?}\t{:?}", child.mv, child.eval, stats);
                drop(child);

                mutex_node = next;
            } else { 
                break;
            }
        }
        println!("best:\n{}", state);
    }

    pub fn solution (&self) -> Result<Node, ()> {
        self.print_best();

        // Find child with highest eval.
        let child = {
            let root = self.root.lock();

            if root.children.len() == 0 {
                panic!("Root has no children");
                //return Err(())
            }

            root.children
                .iter()
                .fold(None, |a, n| -> Option<(_, f32)> {
                    let eval = &n.lock().eval;
                    match a { 
                        None => Some((n.clone(), eval.get())),
                        Some(a) =>
                            if eval.get() > a.1 { Some((n.clone(), eval.get())) } 
                            else { Some(a) }
                    }
                })
                .expect("No children in root, could not find one with highest eval.")
                .0.lock()
                .clone()
        };


        // Debug: print state.
        /*
        let state = self.root_state.read();
        let state = state.clone().apply_move(&child.mv);
        println!("{state}");
        */

        Ok(child)
    }

    
    pub fn advance (&mut self, state: &game::State) {
        // Find child with matching state.
        let mut root_state = self.root_state.write();
        let mut root = self.root.lock();       

        let child = {

            let mut out = None;
            for child in &root.children {
                let child_state = root_state.clone()
                    .apply_move(&child.lock().mv);

                // NOTE: IMPORTANT: This line was changed from before the refactoring. Used to be a
                // function called 'is_child_of(a, b)' that seemed to just check for equality between
                // two states.
                if state.eq(&child_state) {
                    out = Some(child.clone());
                }
            }
            out
        };
        
        *root_state = state.clone();

        // If is a child.
        if let Some(child) = child 
        { // Reassign root & state.
            drop(root);
            self.root = child.clone();
            self.root.lock().expanding = false;
        } else 
        { // Else, reset tree
            *root = Default::default();
        }
    }
}



enum SelectionResult {
    Continue (Arc<Mutex<Node>>),
    Deadend,
    Leaf,
}


#[derive(Debug, Clone, Default)]
pub struct Evaluation {
    present: f32,
    future: Option<f32>
}
impl Evaluation {
    pub fn new (score: f32) -> Self {
        Evaluation {
            present: score, 
            future: None 
        }
    }
    pub fn apply(&mut self, backprop: &Backprop) {
        if let Some(future) = &mut self.future {
            *future = (*future).max(backprop.score);
        } else {
            self.future = Some(backprop.score);
        }
    }

    const INHERITANCE_F: f32 = 0.3;
    fn get(&self) -> f32 {
        if let Some(future) = self.future {
            self.present * (1.0 - Self::INHERITANCE_F) + future * Self::INHERITANCE_F
        } else {
            self.present
        }
    }
}
impl PartialEq for Evaluation {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(&other.get())
    }
}


#[derive(Debug, Clone, Default)]
pub struct Node {
    pub eval: Evaluation,
    pub mv: game::Move,
    pub children: Vec<Arc<Mutex<Node>>>,
    pub expanding: bool,
    pub expansions: u32
}

impl Node {
    fn select (&mut self) -> SelectionResult {
        if self.expanding {
            SelectionResult::Deadend
        } else 
        if self.children.is_empty() {
            SelectionResult::Leaf
        } else {
            let candidates: Vec<_> = self.children.iter().filter(|child| !child.lock().expanding).collect();
            if candidates.is_empty() { return SelectionResult::Deadend }

            self.expansions += 1;

            // let i = rand::thread_rng().gen_range(0..candidates.len()) as usize;
            // SelectionResult::Continue(candidates[i].clone())

            
            let out = candidates.iter().min_by_key(|c| c.lock().expansions).unwrap();
            SelectionResult::Continue((*out).clone())
            
        }
    }

    pub fn expand (&mut self, children: Vec<Arc<Mutex<Node>>>) -> Backprop {
        let backprop = {
            let max_eval = children
                .iter()
                .map(|c| c.lock().eval.clone())
                .max_by(|a, b| a.partial_cmp(&b).unwrap())
                .unwrap()
                .clone();

            Backprop { score: max_eval.get() }
        };

        self.children = children;
        self.expanding = false;

        backprop
    }

    pub fn get_mv (&self) -> &game::Move {
        &self.mv
    }
}

pub fn gen_children (state: &game::State) -> Vec<Node> {
    game::movegen::gen_moves(state)
        .into_iter()
        .map(|mov| state.clone().make_node(mov, game::eval::Mode::Norm))
        .collect()
}

// Prune / Apply Cutoff
pub fn prune_children (mut nodes: Vec<Node>, selection: &Selection) -> Vec<Node> {
    nodes.sort_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
    let n = nodes.len();
    let cutoff_index = { 
        let parent_eval = selection.get_leaf().eval;
        let mut i = 0;
        while i < n && nodes[i].eval.present > CUTOFF_F * parent_eval.get() { i += 1; }
        i.max(20)
    };
    nodes
        .drain(cutoff_index.min(n)..n);

    nodes
}

pub struct Selection {
    list: Vec<Arc<Mutex<Node>>>,
    state: game::State, 
}
impl Selection {
    pub fn new (list: Vec<Arc<Mutex<Node>>>, state: game::State) -> Self {
        Self {
            list, state 
        }
    }

    pub fn get_state(&self) -> &game::State {
        &self.state
    }

    pub fn get_leaf(&self) -> Node {
        self.list
            .last()
            .unwrap()
            .lock()
            .clone()
    }

    pub fn expand(&self, children: Vec<Arc<Mutex<Node>>>) -> Backprop {
        self.list
            .last()
            .unwrap()
            .lock()
            .expand(children)
    }

    // Applys backpropagation update to nodes selected for the relavent expansion.
    // Since self.list is in decending order, applies it in reverse. 
    pub fn backprop(&self, backprop: Backprop) {
        for node in self.list.iter().rev() {
            node.lock().eval.apply(&backprop);
        }
    }
}

pub struct Backprop {
    score: f32,
}
