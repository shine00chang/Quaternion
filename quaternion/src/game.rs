use std::{collections::VecDeque, vec};

pub mod gen;
pub mod eval;

/*
pub use tetron::state::State;
pub use tetron::mov::Move;
pub use tetron::Piece;
use tetron::field::Field; 

// PIECE_MAP for driver. 
pub use tetron::field::PIECE_MAP;
*/

// For WASM. Not sure why. commenting for now.
// pub use tetron::Key;

/*
 * Exports Only Move, Keys, Pieces, State, gen_moves(), evaluate()
 *       output ^--------^          ^---^ Node 
 *                          ^----^ Interface
 */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
    L, R, CW, CCW, Drop
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    L, J, S, Z, T, I, O
}

impl Piece {
    const fn cells_plain(self) -> [(i8, i8); 4] {
        match self {
            Piece::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
            Piece::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
            Piece::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            Piece::Z => [(-1, 1), (0, 1), (0, 0), (1, 0)],
            Piece::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            Piece::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            Piece::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    // TODO: 
    const fn cells(self, r: Rotation) -> [(i8, i8); 4] {
        match self {
            Piece::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            Piece::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Piece::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            Piece::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
            Piece::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
            Piece::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            Piece::Z => [(-1, 1), (0, 1), (0, 0), (1, 0)],
        }
    }
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rotation {
    N, S, W, E
}



#[derive(Clone, Debug)]
pub struct Move {
    x: i8,
    y: i8,
    r: Rotation,
    list: u32,
}
impl Default for Move {
    fn default() -> Self {
        Self { x: 0, y: 0, r: Rotation::N, list: 0 }
    }
}
impl Move {
    // Extracts the keystrokes required to get to this position.
    // TODO:
    pub fn parse_list (&self) -> Vec<Key> {
        vec![]
    }
}



#[derive(Clone, Default, PartialEq)]
struct Board {
    v: [u32; 10]
}



#[derive(Clone, Default, PartialEq)]
pub struct State {
    board: Board,
    queue: VecDeque<Piece>,
    hold: Option<Piece>,
    b2b: u8,
    combo: u8,
}

impl State {
    pub fn apply_move (&mut self, mv: &Move) -> Result<(), ()> {
        Ok(())
    }

    pub fn queue_len (&self) -> usize {
        self.queue.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_move_test () {
        /*
        let mut state = super::State::new();
        state.pieces.push_back(tetron::Piece::I);
        state.pieces.push_back(tetron::Piece::L);
        state.pieces.push_back(tetron::Piece::Z);
        state.pieces.push_back(tetron::Piece::S);
        state.pieces.push_back(tetron::Piece::J);
        state.pieces.push_back(tetron::Piece::O);
        state.pieces.push_back(tetron::Piece::T);

        let mut mv = super::Move::new();
        mv.x = 3;
        mv.r = 2;
        mv.hold = true;
        mv.y = 18;

        state.apply_move(&mv).unwrap();

        println!("{}", state);
        */
    }
}
