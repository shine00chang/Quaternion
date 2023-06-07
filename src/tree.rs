use rand::prelude::*;
use std::sync::Arc; 
use parking_lot::{Mutex, RwLock};
use super::game::{self, Workaround};

pub struct Tree {
    root_state: RwLock<tetron::state::State>,
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
    pub fn select (&self) -> Option<(Arc<Mutex<Node>>, game::State)> {
        let (mut mutex_node, mut state) = {
            let _root = self.root.lock();
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
                    mutex_node = mutex_child;
                },
                SelectionResult::Leaf    => { node.expanding = true; break; }
                SelectionResult::Deadend => return None, 
            }
        }
        Some((mutex_node, state))
    }

    pub fn solution (&self) -> Result<(Node, game::State), ()> {
         // Find child with highest eval.
        let (score, child) = {
            let root = self.root.lock();
            let mut best: Result<(_, _), ()> = Err(());
            for child in &root.children {
                let child_score = child.lock().eval;
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

        // Get solution state
        let child = child.lock().clone();
        let state = {
            let mut state = self.root_state.read().clone();
            state.apply_move(&child.mv)?;
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
    mv: tetron::mov::Move,
    children: Vec<Arc<Mutex<Node>>>,
    expanding: bool
}

impl Default for Node {
    fn default() -> Self {
        Self { 
            eval: 0.0,
            mv: tetron::mov::Move::new(),
            children: vec![],
            expanding: false
        }
    }
}

impl Node {
    fn select (&self) -> SelectionResult {
        if self.expanding {
            SelectionResult::Deadend
        } else 
        if self.children.is_empty() {
            SelectionResult::Leaf
        } else {
            let candidates: Vec<_> = self.children.iter().filter(|child| !child.lock().expanding).collect();
            if candidates.is_empty() { return SelectionResult::Deadend }

            let mut rng = rand::thread_rng();
            let i: usize = (rng.gen::<f64>() * candidates.len() as f64) as usize;
            SelectionResult::Continue(candidates[i].clone())
        }
    }

    pub fn expand (&mut self, children: Vec<Arc<Mutex<Node>>>) {
        self.children = children;
        self.expanding = false;
    }

    pub fn get_mv (&self) -> &game::Move {
        &self.mv
    }
}

pub fn gen_children (state: &game::State) -> Vec<Arc<Mutex<Node>>> {
    // TEMPORARY: Using tetron's gen_moves().
    let map = tetron::gen_moves(state);

    // make moves into node
    let nodes = map.into_iter().map(|(_, mv)| {
        let mut state = state.clone();
        state.apply_move(&mv).expect("Failed to apply move at 'gen_children()'");

        let eval = tetron::evaluate(&state, tetron::EvaluatorMode::Norm);
        let node = Node {
            mv,
            eval,
            children: vec![],
            expanding: false,
        };

        Arc::new(Mutex::new(node))
    }).collect();

    nodes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_children_test () {
        let mut state = game::State::new();
        state.pieces.push_back(tetron::Piece::I);
        state.pieces.push_back(tetron::Piece::L);
        state.pieces.push_back(tetron::Piece::Z);
        state.pieces.push_back(tetron::Piece::S);
        state.pieces.push_back(tetron::Piece::J);
        state.pieces.push_back(tetron::Piece::O);
        state.pieces.push_back(tetron::Piece::T);

        let children = gen_children(&state);
        for child in children {
            let mut state = state.clone();
            let mv = child.lock().mv.clone();
            state.apply_move(&mv).expect("Failed to apply move at 'gen_children()'");

            println!("{:?}\n", mv.parse_list());
            println!("{}\n", state);
        }
    }
}
