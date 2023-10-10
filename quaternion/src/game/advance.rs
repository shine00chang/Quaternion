#[cfg(test)]
mod tests;

use super::{*, eval::evaluate};



impl Move {

    /// Determines if move held. Checks if HOLD is the first key in list.
    pub fn held (&self) -> bool {
        if self.list_len() == 0 { return false }

        let mask = Self::MASK << Self::LEN_W;
        let val  = (self.list & mask) >> Self::LEN_W;
        val == 6
    }
}



impl Board {

    /// Places piece onto map. Does not clear
    fn place (&mut self, piece: Piece, mov: &Move) {
        for (dx, dy) in piece.cells(mov.r) {
            let nx = mov.x + dx;
            let ny = mov.y + dy;
            assert!(nx >= 0 && nx < 10 && ny >= 0);
            assert!(self.v[nx as usize] & (1 << ny) == 0);
            self.v[nx as usize] |= 1 << ny;
        }
    }

    fn clear (&mut self) -> u32 {

        // make mask
        let mut mask = self.v.iter()
            .fold(!0, |a, i| a & i);
        let clears = mask.count_ones();

        // For each i where mask[i] == 1 in decending order, shift the bitseg[i+1..32] left by one, and add it with
        // the bitseg[0..i]. The sum will be the new column.
        while mask != 0 {
            let y = mask.trailing_zeros();
            for col in self.v.iter_mut() {
                let bmask = (1<<y)-1;
                let tmask = (!0 ^ bmask) - (1 << y);
                let bot = *col & bmask;
                let top = *col & tmask;
                *col = bot + (top >> 1);
            }
            mask -= 1 << y;
            mask >>= 1;
        }

        clears
    }

    // Determines if a move is a tspin by 3-corner rule. Assumes the piece is T.
    // Precondition: The move is not clear()'ed yet
    fn is_tspin (&self, mov: &Move) -> bool {
        let corners = 
            if self.occupied(mov.x -1, mov.y -1) { 1 } else { 0 } +
            if self.occupied(mov.x -1, mov.y +1) { 1 } else { 0 } +
            if self.occupied(mov.x +1, mov.y +1) { 1 } else { 0 } +
            if self.occupied(mov.x +1, mov.y -1) { 1 } else { 0 };

        corners >= 3
    }

    fn occupied (&self, x: i8, y: i8) -> bool {
        if x < 0 || x > 9 || y < 0 {
            true 
        } else { 
            self.v[x as usize] & 1 << y != 0 
        }
    }
}



use crate::tree::Node;
impl State {
    
    /// Applies move onto state, returning the resultant child AND THE LINES CLEARED.
    /// Wrapped by `apply-move(..)` for exported interface.
    /// Used by `make_node(..)` to help calculate attack
    fn apply_move_return_clears (mut self, mov: &Move) -> (Self, u32, bool) {

        // Retains the piece placed. Needed for t-spin detection
        let placed = if mov.held() {

            // Get piece, Set hold, Update queue
            let hold = if let Some(hold) = self.hold {
                self.hold = self.queue.pop_front();
                hold
            } else {
                self.hold = Some( self.queue.pop_front().expect("Held without hold, but queue is empty") );
                self.queue.pop_front().expect("Held without hold, but queue only has 1 element")
            };
            
            self.board.place(hold, mov);

            hold
        } else {
            let piece = self.queue.pop_front().expect("Move placed but state's queue was empty.");
            self.board.place(piece, mov);

            piece
        };

        // Check if is tspin. This must be done before 'clear()' is called.
        let is_tspin = placed == Piece::T && self.board.is_tspin(mov);

        // Clear & update combo & b2b
        let clears = self.board.clear();
        if clears > 0 {
            self.combo += 1;
            if is_tspin || clears == 4 { self.b2b += 1 } 
            else { self.b2b = 0 }
        } else {
            self.combo = 0;
            self.b2b = 0;
        }

        (self, clears, is_tspin)
    }


    /// Creates node that will be created from self and the input move.
    /// Applies Move, Calculate evaluation `MetaData` (atk, ds, etc), then evaluate.
    pub fn apply_move_with_stats (mut self, mov: &Move) -> (Self, MoveStats) {

        let (clears, was_tspin) = {
            let (n_state, clears, is_tspin) = self.apply_move_return_clears(&mov);
            self = n_state;
            (clears, is_tspin)
        };

        // === Calculate Evaluation MetaData ===
        // case: no clears
        let mut attacks = if clears == 0 { 
            0 
        // case: basic clears 
        } else if !was_tspin && clears != 4 {
            match clears {
                0 => 0,
                1 => [0, 0, 1, 1, 1, 1, 2, 2, 2, 2][self.combo as usize],
                2 => [1, 1, 1, 1, 2, 2, 2, 2, 3, 3][self.combo as usize],
                3 => [2, 2, 3, 3, 4, 4, 5, 5, 6, 6][self.combo as usize],
                _ => 0
            }
        // case: tspin or tetris
        } else  {
            // TODO: Attack Table
            /*
            let index = if clears == 4 { 0 } else { clears };
            B2B_TABLE[self.b2b as usize][index as usize][self.combo as usize] as u8
            */
            match clears {
                1 => 2,
                2 => 4,
                3 => 6,
                4 => 4,
                _ => unreachable!()
            }
        };

        // Perfect clear
        if clears > 0 && self.board.v.iter().all(|col| *col == 0) {
            attacks += 10;
        }

        let stats = MoveStats {
            attacks,
            ds: clears as u8,
            tspin: self.b2b != 0,
        };

        (self, stats)
    }


    pub fn make_node (mut self, mov: Move, eval_mode: eval::Mode) -> Node {
        let (nstate, stats) = self.apply_move_with_stats(&mov);
        self = nstate;

        // Evaluate
        let eval = evaluate(&self, stats, eval_mode);

        Node {
            eval,
            mv: mov,
            children: vec![],
            expansions: 0,
            expanding: false,
        }
    }


    /// Applies move onto state, returning the resultant child AND THE LINES CLEARED.
    /// Wraps `apply_move_return_clears(..)`
    pub fn apply_move (self, mov: &Move) -> Self {
        self.apply_move_return_clears(mov).0
    }

    pub fn queue_len (&self) -> usize {
        self.queue.len()
    }
}

/// Tetr.io garbage table 
pub const B2B_TABLE: [[[u32; 10]; 4]; 4] = [
    [
        [4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        [2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
        [4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        [6, 7, 9, 10, 12, 13, 15, 16, 18, 19],
    ],
    [
        [5, 6, 7, 8, 10, 11, 12, 13, 15, 16],
        [3, 3, 4, 5, 6, 6, 7, 8, 9, 9],
        [5, 6, 7, 8, 10, 11, 12, 13, 15, 16],
        [7, 8, 10, 12, 14, 15, 17, 19, 21, 22],
    ],
    [
        [6, 7, 9, 10, 12, 13, 15, 16, 18, 19],
        [4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        [6, 7, 9, 10, 12, 13, 15, 16, 18, 19],
        [8, 10, 12, 14, 16, 18, 20, 22, 24, 25],
    ],
    [
        [7, 8, 10, 12, 14, 15, 17, 19, 21, 22],
        [5, 6, 7, 8, 10, 11, 12, 13, 15, 16],
        [7, 8, 10, 12, 14, 15, 17, 19, 21, 22],
        [9, 11, 13, 15, 18, 20, 22, 24, 27, 29],
    ],
];
