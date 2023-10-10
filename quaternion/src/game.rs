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


#[derive(Clone, Debug, Default, PartialEq)]
struct Board {
    v: [u32; 10]
}
impl Board {
    /// Creates board given the text output of a state or board.
    pub fn from_str (s: &str) -> Self {
        let mut b = [[false; 10]; 20];
        let mut y = 0;
        let mut x = 0;
        for ch in s.chars() {
            if ch == '#' || ch == '.' {
                b[y][x] = ch == '#'; 
                x += 1;
                if x == 10 {
                    x = 0;
                    y += 1;
                }
                if y == 20 {
                    break;
                }
            }
        }

        let mut v = [0; 10];
        for x in 0..10 {
            for y in 0..20 {
                if b[19-y][x] {
                    v[x] |= 1 << y;
                }
            }
        }

        Board { v }
    }
}
impl std::fmt::Display for Board {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..20).rev() {
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
impl State {
    /// For testing. Creates a State object given the textual formatting of a state.
    pub fn from_str (s: &str) -> Self {
        let s = s.trim();

        let board = Board::from_str(s);
        let mut queue = VecDeque::new();
        let mut hold = None;
        let mut b2b = 0;
        let mut combo = 0;

        for (i, line) in s.lines().enumerate() {
            // B2B line
            if i == 0 { 
                println!("{line}");
                let last = line.trim().split(' ').last().unwrap();
                println!("{last}");
                b2b = last.parse().expect("could not parse b2b into number");
            }

            // Combo line
            if i == 1 { 
                let last = line.split(' ').last().unwrap();
                combo = last.parse().expect("could not parse combo into number");
            }

            // Hold line
            if i == 2 {
                let ch = line.get(line.len()-1..).unwrap();
                hold = match ch {
                    "L" => Some(Piece::L),
                    "J" => Some(Piece::J),
                    "S" => Some(Piece::S),
                    "Z" => Some(Piece::Z),
                    "T" => Some(Piece::T),
                    "I" => Some(Piece::I),
                    "O" => Some(Piece::O),
                    _ => None,
                }

            }

            // Ignore rest
            if i > 4 && i < 10 {
                let ch = line.get(line.len()-1..).unwrap();
                match ch {
                    "L" => queue.push_back(Piece::L),
                    "J" => queue.push_back(Piece::J),
                    "S" => queue.push_back(Piece::S),
                    "Z" => queue.push_back(Piece::Z),
                    "T" => queue.push_back(Piece::T),
                    "I" => queue.push_back(Piece::I),
                    "O" => queue.push_back(Piece::O),
                    _ => (),
                }
            }
        }
        

        Self {
            board,
            queue,
            hold,
            b2b,
            combo,
        }
    }
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
