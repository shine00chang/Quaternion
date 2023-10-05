use crate::game::*;
use std::collections::HashMap;


/// Dummy for now. Just so that I don't need to revamp quaternion.
pub fn gen_moves (state: &State) -> HashMap<tetron::Field, Move> {
    let x = Piece::L.cells_plain();
    HashMap::new()
}

pub fn gen_moves_new (state: &State) -> Vec<Move> {
    Vec::new()
}

fn shift (board: &Board, mov: Move, dx: i8, conflict_table: &ConflictTable) -> Option<Move> {
    None
}

fn rotate_cw (board: &Board, mov: Move, conflict_table: &ConflictTable) -> Option<Move> {
    None
}

fn rotate_ccw (board: &Board, mov: Move, conflict_table: &ConflictTable) -> Option<Move> {
    None
}
