#[cfg(test)]
mod tests;

use crate::game::*;
use std::collections::{HashMap, HashSet, LinkedList};


/// Dummy for now. Just so that I don't need to revamp quaternion.
pub fn gen_moves (state: &State) -> HashMap<tetron::Field, Move> {
    let x = Piece::L.cells_plain();
    HashMap::new()
}

fn gen_moves_new (board: &Board, piece: Piece) -> Vec<Move> {
    let mut queue: LinkedList<Move> = LinkedList::new();
    let mut set: HashSet<u32> = HashSet::new();
    let mut out: LinkedList<Move> = LinkedList::new();
    let conflict_table = ConflictTable::from(board, piece);

    // Convenience function for updating BFS state & output.
    let mut update = |queue: &mut LinkedList<Move>, set: &mut HashSet<u32>, mov: Move| {
        // Insert to set
        if set.insert(mov.canon()) {
            
            // If touching stack (y-1 conflicts), add to output.
            if conflict_table.touches(&mov) {
                out.push_back(mov.clone())
            }

            // If key list not filled -> have space for more keystrokes, add to queue
            if mov.list_has_space() {
                queue.push_back(mov);
            }
        }
    };

    // Add Spawn
    {
        let spawn = Move {
            x: 4,
            y: 19,
            r: Rotation::N,
            list: 0,
        };

        // If spawn conflicts, return no moves. game over.
        if conflict_table.conflicts(&spawn) {
            return vec![];
        }

        update(&mut queue, &mut set, spawn);
    }

    // BFS
    while let Some(mov) = queue.pop_front() {
        
        // Try each move, call update.
        if let Some(mov) = mov.drop(board, piece) {
            update(&mut queue, &mut set, mov);
        }
        if let Some(mov) = mov.shift(1, &conflict_table) {
            update(&mut queue, &mut set, mov);
        }
        if let Some(mov) = mov.shift(-1, &conflict_table) {
            update(&mut queue, &mut set, mov);
        }
        if let Some(mov) = mov.cw(&conflict_table) {
            update(&mut queue, &mut set, mov);
        }
        if let Some(mov) = mov.ccw(&conflict_table) {
            update(&mut queue, &mut set, mov);
        }
    }


    out.into_iter().collect()
}



impl Move {
    /// Applies Softdrop to the move, outputs if it is different.
    // TODO:
    fn drop (&self, board: &Board, piece: Piece) -> Option<Move> {
        None
    }

    /// Applies Left or Right to the move, outputs if it works and is different.
    // TODO:
    fn shift (&self, dx: i8, conflict_table: &ConflictTable) -> Option<Move> {
        None
    }

    /// Applies Rotates the move, outputs if it works and is different.
    // TODO:
    fn cw (&self, conflict_table: &ConflictTable) -> Option<Move> {
        None
    }

    /// Applies Rotates the move, outputs if it works and is different.
    // TODO:
    fn ccw (&self, conflict_table: &ConflictTable) -> Option<Move> {
        None
    }
}


struct ConflictTable {
    v: [[u32; 10]; 4]
}

impl ConflictTable {
    // TODO:
    fn from (board: &Board, piece: Piece) -> Self {
        Self {
            v: [[0; 10]; 4]
        }
    }

    // TODO:
    fn touches (&self, mov: &Move) -> bool {
        false
    }

    // TODO:
    fn conflicts (&self, mov: &Move) -> bool {
        false
    }
}



impl Move {
    /// Creates the canonical representation of the move. Used for hashmapping.
    fn canon (&self) -> u32 {
        ((self.y as u32) << 16) + ((self.x as u32) << 8) + (self.r as u32)
    }

    /// Returns whether or not the key list is full. 
    // TODO:
    fn list_has_space (&self) -> bool {
        false
    }
}


impl Rotation {
    fn kicktable(piece: &Piece, from: Self, to: Self) -> [(i8, i8); 5] {
        // TODO: Make const evaluation
        const TABLE: [[[(i8, i8); 5]; 4]; 4] = [[[(0, 0); 5]; 4]; 4];

        return TABLE[from as usize][to as usize];
    }
}
