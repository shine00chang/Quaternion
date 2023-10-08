use std::collections::VecDeque;

pub mod movegen;
pub mod advance;
pub mod eval;
pub mod sim;

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
    L, J, S, Z, T, I, O, None
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rotation {
    N, S, E, W
}



#[derive(Clone, Debug)]
pub struct Move {
    x: i8,
    y: i8,
    r: Rotation,
    list: u64,
}


/// Move's metadata. describes statistics of a move after it is applied onto a state.
/// used for evaluation and simulation.
#[derive(Default)]
pub struct MoveStats {
    pub attacks: u8,
    pub ds: u8,
    pub tspin: bool
}

impl Default for Move {
    fn default() -> Self {
        Self { x: 0, y: 0, r: Rotation::N, list: 0 }
    }
}


impl Move {
    const W: u64 = 3;
    const LEN_W: u64 = 5;
    const MASK: u64 = (1<<Self::W) - 1;
    const LIST_CAPACITY: u64 = (64 - Self::LEN_W) / Self::W;

    pub fn list_len (&self) -> u64 {
        self.list & ((1 << Self::LEN_W) -1)
    }

    /// Parses bitset list.
    pub fn parse_list (&self) -> Vec<Key> {
        (0..self.list_len()).map(|i| {
            let shifts = i * Self::W + Self::LEN_W;
            let mask = Self::MASK << shifts;
            let val  = (self.list & mask) >> shifts;
            let key  = match val {
                1 => Key::L,
                2 => Key::R,
                3 => Key::CW,
                4 => Key::CCW,
                5 => Key::Drop,
                6 => Key::Hold,
                _ => panic!("none such key encoding")
            };
            key
        }).collect()
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
                let b = (self.board.v[x] & (1 << (19-y))) != 0;
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
