use super::game::{self, Piece, Move};
use tetron::field::PIECE_MAP;
use std::fmt;

pub mod colors {
    pub const RST: &str = "\x1b[0m";
    pub const BLD: &str = "\x1b[1m";
    pub const HLT: &str = "\x1b[48;5;226m";
    macro_rules! piece_color {
        ($p: expr) => {
            match $p {
                Piece::None => "\x1b[47;1m", // white
                Piece::J => "\x1b[48;5;20m", // blue
                Piece::L => "\x1b[48;5;208m", // bright red / orange
                Piece::S => "\x1b[48;5;46m", // green
                Piece::Z => "\x1b[48;5;9m", // red
                Piece::T => "\x1b[45;1m", // magenta
                Piece::I => "\x1b[48;5;51m", // cyan
                Piece::O => "\x1b[48;5;226m", // yellow
            }
        };
    }
    pub(crate) use piece_color;
}
use colors::*;

#[derive(Clone)]
pub(super) struct SimState {
    m: [[Piece; 10]; 20],
    state: game::State
}

impl SimState {
    pub fn new (state: game::State) -> Self {
        Self {
            m: [[Piece::None; 10]; 20],
            state
        }
    }

    pub fn get_state (&self) -> &game::State { &self.state }

    // Given a move and the bot's returned state, updates the board.
    // Panics if the move does not result in the state given.
    pub fn advance (&mut self, m: &Move, next_state: &game::State) {
        // Apply Move
        { 
            let s = &self.state;
            let p = &if !m.hold { s.pieces[0] } 
                    else {
                        if s.hold == Piece::None { s.pieces[1] } else { s.hold } 
                    };


            let map: &[u16; 5] = &PIECE_MAP[*p as usize][m.r as usize];
            let n: i8 = if *p == Piece::I {5} else {3};
            let c_x: i8 = m.x - n/2;
            let c_y: i8 = m.y - n/2;
            
            for y in 0..n {
                // The bits representing a single row of the piece map
                let bitseg: u16 = map[y as usize].reverse_bits() >> (16 - n);
                // UNLEASH LATER :biflush: let bitseg: u16 = PIECE_MAP[*p as usize][m.r as usize][y as usize].reverse_bits() >> (16 - n);
                //println!("c_x: {c_x}, map: {:09b}, bitseg: {:05b}\r", PIECE_MAP[*p as usize][m.r as usize], bitseg);

                // If empty row on piece map
                if bitseg == 0 {
                    continue;
                }
                // If out of board on upper edge
                if  c_y + y < 0 {
                    panic!("@ Field.apply_move: out of board on upper edge");
                }
                // If out of board on bottom edge
                if c_y + y >= 20 {
                    panic!("@ Field.apply_move: out of board on bottom edge");
                }
                // If out of board on left edge
                if c_x < 0 && bitseg & ((1 << (-c_x)) - 1) > 0  {
                    panic!("@ Field.apply_move: out of board on left edge");
                }
                // Shift according to c_x
                let bitseg = if c_x > 0 { bitseg << c_x } else { bitseg >> -c_x };
                //dev_log!("c_x: {}, final bitseg: {:05b}", c_x, bitseg);
                // If out of board on right edge
                if bitseg > (1 << 10)-1 {
                    panic!("@ Field.apply_move: out of board on right edge");
                }
                for x in 0..10 {
                    if (1 << x) & bitseg > 0 {
                        self.m[(c_y + y) as usize][x] = *p;
                    }
                }
            };
        }

        // Clear 
        {
            let mut clears: usize = 0;
            for y in (0..20).rev() {
                let mut clear: bool = true;
                for x in 0..10 {
                    if self.m[y][x] == Piece::None {
                        clear = false;
                    }
                    if clears > 0 {
                        self.m[y+clears][x] = self.m[y][x];
                        self.m[y][x] = Piece::None;
                    }
                }
                if clear {
                    clears += 1;
                }
            }
        }

        self.state = next_state.clone();
        // Check
        {
            for i in 0..20 {
                let row = {
                    let mut x = 0;
                    for j in 0..10 { if !matches!(self.m[i][j], Piece::None) { x += 1 << j } }
                    x as u16
                };
                if row != self.state.field.m[i] {
                    panic!("sim::State::advance() given a move that does not result in the given state.");
                }
            }
        }
    }
}

impl fmt::Display for SimState {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..20 {
            for x in 0..10 {
                if self.m[y][x] != Piece::None {
                    let c = piece_color!(self.m[y][x]);
                    write!(f, "{}  {}", c, RST)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            print!(" ");
            match y {
                0 => print!("b2b:   {BLD}{:>2}{RST}", self.state.props.b2b),
                1 => print!("combo: {BLD}{:>2}{RST}", self.state.props.combo),
                3 => print!("hold:  {BLD}{:?}{RST}", self.state.hold),
                4 => print!("queue:"),
                5..=9 => if self.state.pieces.len() > y-5 {
                    print!("{BLD}{:?}{RST}", self.state.pieces[y-5])
                },
                _ => ()
            };
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}
