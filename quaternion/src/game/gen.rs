#[cfg(test)]
mod tests;

use crate::game::*;
use std::collections::{HashSet, LinkedList};


/// Returns a list of all possible moves from this current state. Includes hold piece.
/// Wraps private 'gen_moves_one(..)'.
pub fn gen_moves (state: &State) -> Vec<Move> {
    let piece = state.queue
        .front()
        .expect("State has no pieces in queue. cannot generate moves.")
        .clone();

    gen_moves_one(&state.board, piece)
        .into_iter()
        .chain(
            if let Some(hold) = state.hold {
                gen_moves_one(&state.board, hold)
            } else { vec![] }.into_iter()
        )
        .collect()
}

/// Returns a list of all possible moves from a board for a single piece.
/// Wrapped by 'gen_moves(..)' for exported interface.
fn gen_moves_one (board: &Board, piece: Piece) -> Vec<Move> {
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
    fn shift (&self, dx: i8, conflict_table: &ConflictTable) -> Option<Move> {
        let mut nmov = Move {
            x: self.x + dx,
            ..*self
        };
        if !conflict_table.conflicts(&nmov) {
            nmov.add_key(if dx > 0 { &Key::R } else { &Key::L });
            Some(nmov)
        } else {
            None
        }
    }

    /// Applies Rotates the move, outputs if it works and is different.
    fn cw (&self, conflict_table: &ConflictTable) -> Option<Move> {
        let nr = match self.r {
            Rotation::N => Rotation::E,
            Rotation::S => Rotation::W,
            Rotation::E => Rotation::S,
            Rotation::W => Rotation::N,
        };
        self.rotate(conflict_table, self.r, nr)
            .map(|mut mov| {
                mov.add_key(&Key::CW);
                mov
            })
    }

    /// Applies Rotates the move, outputs if it works and is different.
    fn ccw (&self, conflict_table: &ConflictTable) -> Option<Move> {
        let nr = match self.r {
            Rotation::N => Rotation::W,
            Rotation::S => Rotation::E,
            Rotation::E => Rotation::N,
            Rotation::W => Rotation::S,
        };
        self.rotate(conflict_table, self.r, nr)
            .map(|mut mov| {
                mov.add_key(&Key::CCW);
                mov
            })
    }

    fn rotate(&self, conflict_table: &ConflictTable, from: Rotation, to: Rotation) -> Option<Move> {
        let kicks = Rotation::kicktable(conflict_table.piece, from, to);

        for kick in kicks {
            let nmov = Move {
                x: self.x + kick.0,
                y: self.y + kick.1,
                ..*self
            };
            if !conflict_table.conflicts(&nmov) {
                return Some(nmov)
            }
        }
        None
    }
}


struct ConflictTable {
    v: [[u32; 10]; 4],
    pub piece: Piece,
}

impl ConflictTable {
    // TODO:
    fn from (board: &Board, piece: Piece) -> Self {
        Self {
            v: [[0; 10]; 4],
            piece,
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

    const W: u64 = 3;
    const LEN_W: u64 = 4;
    const MASK: u64 = (1<<Self::W) - 1;
    const LIST_CAPACITY: u64 = (64 - Self::LEN_W) / Self::W;
    pub fn list_len (&self) -> u64 {
        self.list & Self::MASK
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

    /// Adds key to bitset list.
    fn add_key (&mut self, key: &Key) {
        if !self.list_has_space() {
            panic!("tried adding to key list when list full.");
        }

        let v: u64 = match key {
            Key::L    => 1,
            Key::R    => 2,
            Key::CW   => 3,
            Key::CCW  => 4,
            Key::Drop => 5,
            Key::Hold => 6,
        };
        let index = self.list_len();
        
        self.list += v << (index * Self::W + Self::LEN_W);
        self.list += 1; // Increment counter
    }

    /// Returns whether or not the key list is full. 
    fn list_has_space (&self) -> bool {
        self.list_len() < Self::LIST_CAPACITY
    }
}


/// Workaround for 'for' loops. Expands a for loop.
macro_rules! expand_for {
    (for $ident:ident in [$($item:expr),*] $block:expr) => {
        [
            $(
                {
                    let $ident = $item;
                    $block
                },
            )*
        ]
    };
}

/// Workaround for 'for' loops. Expands a for loop over all pieces.
macro_rules! for_each_piece {
    ($ident:ident in $block:expr) => {
        expand_for!(for $ident in [Piece::L, Piece::J, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O] $block) 
    };
}

/// Workaround for 'for' loops. Expands a for loop over all rotations.
macro_rules! for_each_rotation {
    ($ident:ident in $block:expr) => {
        expand_for!(for $ident in [Rotation::N, Rotation::S, Rotation::E, Rotation::W] $block) 
    };
}

// Const Evaluation of Kicktable
impl Rotation {
    const fn offsets(piece: Piece, rotation: Rotation) -> [(i8, i8); 5] {
        match piece {
            Piece::O => match rotation {
                Rotation::N => [(0, 0); 5],
                Rotation::E => [(0, -1); 5],
                Rotation::S => [(-1, -1); 5],
                Rotation::W => [(-1, 0); 5],
            },
            Piece::I => match rotation {
                Rotation::N => [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
                Rotation::E => [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
                Rotation::S => [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
                Rotation::W => [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
            },
            _ => match rotation {
                Rotation::N => [(0, 0); 5],
                Rotation::E => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                Rotation::S => [(0, 0); 5],
                Rotation::W => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
            },
        }
    }
    const fn make_kicks() -> [[[[(i8, i8); 5]; 4]; 4]; 7] {
       for_each_piece!(piece in {
            for_each_rotation!(from in {
                for_each_rotation!(to in {
                    let mut from = Rotation::offsets(piece, from);
                    let to = Rotation::offsets(piece, to);
                    let mut i = 0;
                    while i < from.len() {
                        from[i].0 -= to[i].0;
                        from[i].1 -= to[i].1;
                        i += 1;
                    }
                    from
                })
            })
        })
    }

    fn kicktable(piece: Piece, from: Self, to: Self) -> [(i8, i8); 5] {
        Rotation::make_kicks()[piece as usize][from as usize][to as usize]
    }
}


impl Piece {
    const fn rotate (p: (i8, i8), r: Rotation) -> (i8, i8) {
        match r {
            Rotation::N => ( p.0,  p.1),
            Rotation::S => (-p.0, -p.1),
            Rotation::E => ( p.1, -p.0),
            Rotation::W => (-p.1,  p.0),
        }
    }

    const fn cells (self, r: Rotation) -> [(i8, i8); 4] {
        match self {
            Piece::L => expand_for!(for pair in [(-1, 0), (0, 0), (1, 0), ( 1, 1)] Self::rotate(pair, r)),
            Piece::J => expand_for!(for pair in [(-1, 0), (0, 0), (1, 0), (-1, 1)] Self::rotate(pair, r)),
            Piece::S => expand_for!(for pair in [(-1, 0), (0, 0), (0, 1), ( 1, 1)] Self::rotate(pair, r)),
            Piece::Z => expand_for!(for pair in [(-1, 1), (0, 1), (0, 0), ( 1, 0)] Self::rotate(pair, r)),
            Piece::T => expand_for!(for pair in [(-1, 0), (0, 0), (1, 0), ( 0, 1)] Self::rotate(pair, r)),
            Piece::I => expand_for!(for pair in [(-1, 0), (0, 0), (1, 0), ( 2, 0)] Self::rotate(pair, r)),
            Piece::O => expand_for!(for pair in [( 0, 0), (1, 0), (0, 1), ( 1, 1)] Self::rotate(pair, r)),
        }
    }
}
