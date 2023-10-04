use std::sync::Arc; 
use parking_lot::{Mutex, RwLock};
use super::game::{self, Workaround};

use crate::{gen, eval};

const CUTOFF_F: f32 = 0.2;

pub struct Tree {
    root_state: RwLock<game::State>,
    root: Arc<Mutex<Node>>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            root_state: RwLock::new( game::State::new() ),
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
                    state.apply_move(child.get_mv()).expect("Could not apply move");
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

    /*
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
                            let eval = n.lock().eval;
                            if eval > a.1 { (n.clone(), eval) } 
                            else { a }
                        }).0)
                };

            if let Ok(next) = best {
                drop(node);
                let child = next.lock();
                state.apply_move(child.get_mv()).expect("Could not apply move");
                drop(child);

                mutex_node = next;
            } else { 
                break;
            }
        }
        println!("best:\n{}", state);
    }
    */

    pub fn solution (&self) -> Result<(Node, game::State), ()> {
        //self.print_best();

        // Find child with highest eval.
        let child = {
            let root = self.root.lock();

            if root.children.len() == 0 {
                panic!("Root has no children");
                //return Err(())
            }

            root.children
                .iter()
                .fold((Arc::new(Mutex::new(Node::default())), f32::MIN), |a, n| {
                    let eval = n.lock().eval;
                    if eval > a.1 { (n.clone(), eval) } 
                    else { a }
                })
                .0.lock().clone()
        };

        // Get solution state
        let child = child;
        let state = {
            let mut state = self.root_state.read().clone();
            state.apply_move(&child.mv).expect("could not apply move");
            state
        };
        Ok((child, state))
    }
    
    pub fn advance (&mut self, state: &game::State) {
        // Find child with matching state.
        let mut root_state = self.root_state.write();
        let mut root = self.root.lock();       

        let child = {

            let mut out = None;
            for child in &root.children {
                let mut child_state = root_state.clone();
                child_state.apply_move(&child.lock().mv).expect("Apply move failed on Tree::advance()");

                if is_child_of(state, &child_state) {
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

fn is_child_of (a: &game::State, b: &game::State) -> bool {
    for i in 0..20 {
        if a.field.m[i] != b.field.m[i] {
            return false;
        }
    }
    if a.hold != b.hold { return false }
    if a.pieces.len() < b.pieces.len() { return false }
    for i in 0..b.pieces.len() {
        if a.pieces[i] != b.pieces[i] {
            return false
        }
    }
    return true;
}

enum SelectionResult {
    Continue (Arc<Mutex<Node>>),
    Deadend,
    Leaf,
}

pub type Evaluation = f32;

#[derive(Debug, Clone)]
pub struct Node {
    eval: Evaluation,
    mv: game::Move,
    children: Vec<Arc<Mutex<Node>>>,
    expanding: bool,
    expansions: u32
}

impl Default for Node {
    fn default() -> Self {
        Self { 
            eval: f32::MIN,
            mv: game::Move::new(),
            children: vec![],
            expanding: false,
            expansions: 0
        }
    }
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

            /*
            let mut rng = rand::thread_rng();
            let i: usize = (rng.gen::<f64>() * candidates.len() as f64) as usize;
            SelectionResult::Continue(candidates[i].clone())
            */ 

            let out = candidates.iter().min_by_key(|c| c.lock().expansions).unwrap();
            SelectionResult::Continue((*out).clone())
        }
    }

    pub fn expand (&mut self, children: Vec<Arc<Mutex<Node>>>) -> Backprop {
        // TODO make backprop
        let backprop = {
            let evals: Vec<_> = 
                children
                    .iter()
                    .map(|c|  c.lock().eval)
                    .collect();

            let max_eval = 
                *evals
                    .iter()
                    .max_by(|a, b| a.partial_cmp(&b).unwrap())
                    .unwrap_or(&-10000.0);

            self.eval = max_eval;
            Backprop { score: max_eval }
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
    // TEMPORARY: Using game's gen_moves().
    gen::gen_moves(state)
        .into_iter()
        .map(|(_, mv)| {
            let mut state = state.clone();
            state.apply_move(&mv).expect("Failed to apply move at 'gen_children()'");

            let eval = eval::evaluate(&state, eval::Mode::Norm);
            Node {
                mv,
                eval,
                children: vec![],
                expanding: false,
                expansions: 0
            }
        })
        .collect()
}

// Prune / Apply Cutoff
pub fn prune_children (mut nodes: Vec<Node>, selection: &Selection) -> Vec<Node> {
    nodes
        .sort_by(|a, b| {
            b.eval.total_cmp(&a.eval)
        });
    let n = nodes.len();
    let cutoff_index = { 
        let parent_eval = selection.get_leaf().eval;
        let mut i = 0;
        while i < n && nodes[i].eval > CUTOFF_F * parent_eval { i += 1; }
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
        for node in self.list[0..self.list.len()-1].iter().rev() {
            backprop.apply_to(&mut node.lock());
        }
    }
}

pub struct Backprop {
    score: f32,
}

impl Backprop {
    pub fn apply_to (&self, node: &mut Node) {
        // TODO
        node.eval = self.score.max(node.eval);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_children_test () {
        let mut state = game::State::new();
        state.pieces.push_back(game::Piece::I);
        state.pieces.push_back(game::Piece::L);
        state.pieces.push_back(game::Piece::Z);
        state.pieces.push_back(game::Piece::S);
        state.pieces.push_back(game::Piece::J);
        state.pieces.push_back(game::Piece::O);
        state.pieces.push_back(game::Piece::T);

        let children = gen_children(&state);
        for child in children {
            let mut state = state.clone();
            let mv = child.mv.clone();
            state.apply_move(&mv).expect("Failed to apply move at 'gen_children()'");

            println!("{:?}\n", mv.parse_list());
            println!("{}\n", state);
        }
    }
}
