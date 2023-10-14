use super::*;


pub const RST: &str = "\x1b[0m";
pub const BLD: &str = "\x1b[1m";
pub const HLT: &str = "\x1b[48;5;226m";

impl Piece {
    pub fn color (&self) -> &'static str {
        match *self {
            Piece::None => "\x1b[47;1m", // white
            Piece::J => "\x1b[48;5;20m", // blue
            Piece::L => "\x1b[48;5;208m", // bright red / orange
            Piece::S => "\x1b[48;5;46m", // green
            Piece::Z => "\x1b[48;5;9m", // red
            Piece::T => "\x1b[45;1m", // magenta
            Piece::I => "\x1b[48;5;51m", // cyan
            Piece::O => "\x1b[48;5;226m", // yellow
        }
    }
}

pub struct SimState {
    pub state: State,
    v: [[Piece; 10]; 20],
    bag: Vec<Piece>,
}

impl std::fmt::Display for SimState {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..20 {
            for x in 0..10 {
                if self.v[19-y][x] != Piece::None {
                    let c = self.v[19-y][x].color();
                    write!(f, "{}  {}", c, RST)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, " ")?;
            match y {
                0 => write!(f, "b2b:   {BLD}{:>2}{RST}", self.state.b2b)?,
                1 => write!(f, "combo: {BLD}{:>2}{RST}", self.state.combo)?,
                3 => write!(f, "hold:  {BLD}{}{RST}",
                            if let Some(hold) = self.state.hold { format!("{:?}", hold) } else { "none".to_owned() })?,
                4 => write!(f, "queue:")?,
                5..=9 => if self.state.queue.len() > y-5 {
                    write!(f, "{BLD}{:?}{RST}", self.state.queue[y-5])?
                },
                _ => ()
            };
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}


impl SimState {
    pub fn new () -> Self {
        let mut out = Self {
            v: [[Piece::None; 10]; 20],
            bag: vec![],
            state: Default::default()
        };
        out.draw();
        out
    }

    pub fn get_state (&self) -> &State {
        &self.state
    }

    /// Creates SimState from textual representation.
    pub fn from_str (s: &str) -> Self {
        let s = s.trim();

        let state = State::from_str(s);
        let bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O]
            .into_iter()
            .filter(|piece| !state.queue.contains(piece))
            .collect();
        let mut v = [[Piece::None; 10]; 20];
        
        for x in 0..10 {
            for y in 0..20 {
                if state.board.v[x] & 1 << y != 0 {
                    v[y][x] = Piece::O;
                }
            }
        }

        Self {
            state,
            v,
            bag,
        }
    }

    pub fn equals (&self, s: &str) -> bool {
        let s = s.trim();

        let board = Board::from_str(s);

        for x in 0..10 {
            for y in 0..20 {
                if board.v[x] & 1 << y != 0 && self.v[y][x] == Piece::None {
                    return false;
                }
            }
        }
        return true;
    }

    /// Draws from bag, populates piece queue. Auto re-fill of bag.
    pub fn draw (&mut self) {
        while self.state.queue.len() < 6 {
            if self.bag.is_empty() {
                self.bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O];
            }
            let i = rand::random::<usize>().clamp(0, self.bag.len()-1);
            let p = self.bag.remove(i);
            self.state.queue.push_back(p);
        }
    }

    /// Generates garbage lines
    pub fn gen_garbage<R> (&mut self, lines: usize, rand: &mut R) 
    where 
        R: rand::Rng 
    {
        let lines = lines.min(10);

        let i = rand.gen_range(0..9);
        let mut garbage_row = [Piece::O; 10];
        garbage_row[i as usize] = Piece::None;

        for y in (0..20).rev() {
            if y < lines {
                self.v[y] = garbage_row;
            } else {
                self.v[y] = self.v[y-lines];
            }
        }

        for x in 0..10 {
            if x == i as usize {
                self.state.board.v[x] <<= lines;
            } else {
                // We want the bits added by the shift to be ones, thus the double negation.
                self.state.board.v[x] = !(!self.state.board.v[x] << lines);
            }
        }
    }

    /// Advances state into next given move.
    pub fn advance (mut self, mov: &Move) -> (Self, MoveStats) {
        let placed = if mov.held() {
            // Get piece, Set hold, Update queue
            if let Some(hold) = self.state.hold {
                hold
            } else {
                *self.state.queue.get(1).expect("Sim State: Held without hold, but queue only has 1 element")
            }
        } else {
            *self.state.queue.front().expect("Sim State: Move placed but state's queue was empty.")
        };

        // Places on colored V
        for (dx, dy) in placed.cells(mov.r) {
            let nx = mov.x + dx;
            let ny = mov.y + dy;
            assert!(nx >= 0 && nx < 10 && ny >= 0);
            assert!(self.v[ny as usize][nx as usize] == Piece::None);
            self.v[ny as usize][nx as usize] = placed;
        }
        // Clear lines for colored V 
        let mut clears = 0;
        for y in 0..20 {
            if self.v[y].iter().fold(true, |a, cell| a && *cell != Piece::None) {
                clears += 1;
            } else if clears != 0 {
                self.v[y-clears] = self.v[y];
                self.v[y] = [Piece::None; 10];
            }
        }

        // Apply onto State, get MoveStats
        let (state, move_stats) = self.state.apply_move_with_stats(mov);
        self.state = state;


        self.draw();

        (self, move_stats)
    }
}
