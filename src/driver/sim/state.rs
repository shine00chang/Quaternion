use rand::prelude::*;
use std::fmt;

use quattron::game; 
use crate::colors::*;

#[derive(Clone)]
pub struct State {
    m: [[game::Piece; 10]; 20],
    state: game::State,
    bag: Vec<game::Piece>
}

impl State {
    pub fn new (state: game::State) -> Self {
        Self {
            m: [[game::Piece::None; 10]; 20],
            state,
            bag: vec![]
        }
    }

    // Update state's field based on map, then return
    pub fn get_state (&self) -> &game::State { &self.state }

    // Given a move and the bot's returned state, updates the board.
    // Panics if the move does not result in the state given.
    pub fn advance (&mut self, m: &game::Move, next_state: &game::State) {
        // Apply Move
        { 
            let s = &self.state;
            let p = &if !m.hold { s.pieces[0] } 
                    else {
                        if s.hold == game::Piece::None { s.pieces[1] } else { s.hold } 
                    };


            let map: &[u16; 5] = &game::PIECE_MAP[*p as usize][m.r as usize];
            let n: i8 = if *p == game::Piece::I {5} else {3};
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
                    if self.m[y][x] == game::Piece::None {
                        clear = false;
                    }
                    if clears > 0 {
                        self.m[y+clears][x] = self.m[y][x];
                        self.m[y][x] = game::Piece::None;
                    }
                }
                if clear {
                    clears += 1;
                }
            }
        }

        self.state = next_state.clone();
        // Check if move results in expected state
        {
            for i in 0..20 {
                let row = {
                    let mut x = 0;
                    for j in 0..10 { if !matches!(self.m[i][j], game::Piece::None) { x += 1 << j } }
                    x as u16
                };
                if row != self.state.field.m[i] {
                    panic!("sim::State::advance() given a move that does not result in the given state.");
                }
            }
        }
    }

    pub fn gen_garbage<R> (&mut self, rng: &mut R, lines: usize) 
    where 
        R: Rng
    {
        let lines = lines.min(10);
        static mut PREV: u8 = 0;

        let i: u8 = unsafe {
            let mut i: u8 = PREV;
            while i == PREV { i = rng.gen_range(0..10); }
            PREV = i;
            i
        };
        let nrow: u16 = ((1 << 10) - 1) - (1 << i);

        for y in lines..20 {
            self.m[y-lines] = self.m[y-lines];
        }

        for y in (20-lines)..20 {
            self.state.field.m[y] = nrow;
            for x in 0..10 {
                self.m[y][x] = if nrow & 1 << x > 0 {game::Piece::L} else {game::Piece::None};
            }
        }
    }

    pub fn draw<R> (&mut self, rng: &mut R) 
    where 
        R: Rng
    {
        while self.state.pieces.len() < 6 {
            if self.bag.is_empty() {
                self.bag.push(game::Piece::J);
                self.bag.push(game::Piece::L);
                self.bag.push(game::Piece::S);
                self.bag.push(game::Piece::Z);
                self.bag.push(game::Piece::T);
                self.bag.push(game::Piece::I);
                self.bag.push(game::Piece::O);
            }
            let i = rng.gen_range(0..self.bag.len());
            let p = self.bag.remove(i);
            self.state.pieces.push_back(p);
        }
    }
}

impl fmt::Display for State {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..20 {
            for x in 0..10 {
                if self.m[y][x] != game::Piece::None {
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
