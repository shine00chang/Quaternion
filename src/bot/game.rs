pub use tetron::state::State;
pub use tetron::mov::Move;
pub use tetron::Piece;
pub use tetron::field::PIECE_MAP;

pub trait Workaround {
    fn apply_move (&mut self, mv: &Move) -> Result<(), ()>;
}

impl Workaround for State {
    fn apply_move (&mut self, mv: &Move) -> Result<(), ()> {
        let piece   = self.pieces.front().unwrap();
        let hold    = &if self.hold == Piece::None { self.pieces[1] } else { self.hold };

        let field = self.field.apply_move(&mv, piece, hold)?;
        *self = self.clone_as_child(field, &mv);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_move_test () {
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
    }
}
