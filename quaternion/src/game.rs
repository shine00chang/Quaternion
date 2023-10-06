use std::{collections::VecDeque, vec};

pub mod gen;
pub mod eval;

/*
 * Exports Only Move, Keys, Pieces, State, gen_moves(), evaluate()
 *       output ^--------^          ^---^ Node 
 *                          ^----^ Interface
 */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
    L, R, CW, CCW, Drop, Hold
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    L, J, S, Z, T, I, O
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rotation {
    N, S, E, W
}



#[derive(Clone, Debug)]
pub struct Move {
    x: i8,
    y: i8,
    r: Rotation,
    list: u64,
}
impl Default for Move {
    fn default() -> Self {
        Self { x: 0, y: 0, r: Rotation::N, list: 0 }
    }
}



#[derive(Clone, Default, PartialEq)]
struct Board {
    v: [u32; 10]
}
impl std::fmt::Display for Board {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..20 {
            for x in 0..10 {
                let b = (self.v[x] & (1 << y)) != 0;
                write!(f, "{} ", if b { '#' } else { '.' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


#[derive(Clone, Default, PartialEq)]
pub struct State {
    board: Board,
    queue: VecDeque<Piece>,
    hold: Option<Piece>,
    b2b: u8,
    combo: u8,
}
impl std::fmt::Display for State {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..20 {
            for x in 0..10 {
                let b = (self.board.v[x] & (1 << y)) != 0;
                write!(f, "{} ", if b { '#' } else { '.' })?;
            }
            print!(" ");
            match y {
                0 => write!(f, "b2b:   {:>2}", self.b2b)?,
                1 => write!(f, "combo: {:>2}", self.combo)?,
                3 => write!(f, "hold:  {:?}", self.hold)?,
                4 => write!(f, "queue:")?,
                5..=9 => if self.queue.len() > y-5 {
                    write!(f, "{:?}", self.queue[y-5])?
                },
                _ => ()
            };
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl State {
    // TODO:
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
