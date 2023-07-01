use std::time::Duration;
use std::collections::VecDeque;

use wasm_thread as thread;
use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    J = 1,
    L = 2,
    S = 3,
    Z = 4,
    T = 5,
    I = 6,
    O = 7,
    Some = 8,
    None = 0,
}
impl Piece {
    fn to_bot (self) -> quaternion::game::Piece {
        match self {
            Piece::T => quaternion::game::Piece::T,
            Piece::I => quaternion::game::Piece::I,
            Piece::L => quaternion::game::Piece::L,
            Piece::J => quaternion::game::Piece::J,
            Piece::S => quaternion::game::Piece::S,
            Piece::Z => quaternion::game::Piece::Z,
            Piece::O => quaternion::game::Piece::O,
            Piece::Some => quaternion::game::Piece::O, // Inaccuracy, but should not have repercussions
            Piece::None => quaternion::game::Piece::None,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Key {
    None        = 0,
    Left        = 1,
    Right       = 2,
    Cw          = 3,
    Ccw         = 4,
    _180        = 5,
    HardDrop    = 6,
    SoftDrop    = 7,
    Hold        = 8
}

#[wasm_bindgen]
pub struct Input {
    board: Vec<Piece>,
    pieces: Vec<Piece>,
    hold: Piece,
}

#[wasm_bindgen]
impl Input {
    #[wasm_bindgen]
    pub fn new () -> Self {
        Self {
            board:  vec![Piece::None; 200],
            pieces: vec![Piece::None; 6],
            hold:   Piece::None,
        }
    }

    #[wasm_bindgen]
    pub fn set_board(&mut self, x: usize, y: usize, p: Piece) {
        self.board[y*10 + x] = p;
    }

    #[wasm_bindgen]
    pub fn set_pieces(&mut self, i: usize, p: Piece) {
        self.pieces[i] = p;
    }

    #[wasm_bindgen]
    pub fn set_hold(&mut self, p: Piece) {
        self.hold = p;
    }

    fn parse (&self) -> quaternion::game::State {
        // Set Board
        let mut state = quaternion::game::State::new();

        for y in 0..20 {
            for x in 0..10 {
                if self.board[y*10 +x] != Piece::None {
                    state.field.m[y] += 1 << x;
                }
            }
        }

        // Set Piece
        for p in &self.pieces {
            state.pieces.push_back(p.to_bot());
        }

        // Set Hold
        state.hold = self.hold.to_bot();

        console_log!("Input State (wasm-driver):\n{}", state);
        state
    }

    // Testing Purposes
    #[wasm_bindgen]
    pub fn test (&self, p: Piece) {
        console_log!("== Test ==: Hello From Rust!!");
        console_log!("== Test ==: Piece: {:?}", p);
    }
}

#[wasm_bindgen]
pub struct Prog {
    bot: quaternion::Quaternion,
}

#[wasm_bindgen]
impl Prog {
    #[wasm_bindgen]
    pub fn new (threads: u32) -> Self {
        Self {
            bot: quaternion::Quaternion::with_threads(threads),
        }
    }

    #[wasm_bindgen]
    pub fn test (&self) {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(1));
                    console_log!("Done {i}");
                })
            })
            .collect();

        thread::spawn(move || {
            for h in handles {
                h.join().unwrap();
            }
            console_log!("Rust join thread done.");
        });
        console_log!("Rust main thread done.");
    }

    #[wasm_bindgen]
    pub fn stop (&self) {
        self.bot.stop()
    }

    #[wasm_bindgen]
    pub fn start (&self) {
        self.bot.start()
    }

    #[wasm_bindgen]
    pub fn end (self) {
        self.bot.end()
    }

    #[wasm_bindgen]
    pub fn solution (&self) -> Output {
        let (mv, state) = self.bot.solution();
        print_state(state);
        Output::new(mv)
    }

    #[wasm_bindgen]
    pub fn advance (&self, input: Input) {
        self.bot.advance(input.parse());
    }
}


#[wasm_bindgen]
pub struct Output {
    list: VecDeque<Key>
}

impl Output {
    pub fn new (mov: quaternion::game::Move) -> Self {
        let mut list: VecDeque<Key> = VecDeque::new();
        mov.parse_list().iter().for_each(|k| match k {
            quaternion::game::Key::Left       => list.push_back(Key::Left),
            quaternion::game::Key::Right      => list.push_back(Key::Right),
            quaternion::game::Key::DASLeft    => for _ in 0..5 { list.push_back(Key::Left) },
            quaternion::game::Key::DASRight   => for _ in 0..5 { list.push_back(Key::Left) },
            quaternion::game::Key::Cw         => list.push_back(Key::Cw),
            quaternion::game::Key::Ccw        => list.push_back(Key::Ccw),
            quaternion::game::Key::_180       => list.push_back(Key::_180),
            quaternion::game::Key::HardDrop   => list.push_back(Key::HardDrop),
            quaternion::game::Key::SoftDrop   => list.push_back(Key::SoftDrop),
            quaternion::game::Key::Hold       => list.push_back(Key::Hold)
        });
        Self { list }
    }
    pub fn none () -> Self {
        Self { list: VecDeque::new() }
    }
}

#[wasm_bindgen]
impl Output {
    #[wasm_bindgen]
    pub fn next (&mut self) -> Key {
        self.list.pop_front().unwrap_or(Key::None)
    }
}


fn print_state (s: quaternion::game::State) {

    let mut str = String::from("");
    for y in 0..20 {
        for x in 0..10 {
            let b: bool = (s.field.m[y] & (1 << x)) >> x == 1;
            if b {
                str.push_str(&format!("# "));
            } else {
                str.push_str(&format!(". "));
            }
        }
        str.push_str(" ");
        match y {
            0 => str.push_str(&format!("b2b:   {:>2}", s.props.b2b)),
            1 => str.push_str(&format!("combo: {:>2}", s.props.combo)),
            3 => str.push_str(&format!("hold:  {:?}", s.hold)),
            4 => str.push_str(&format!("queue:")),
            5..=9 => if s.pieces.len() > y-5 {
                str.push_str(&format!("{:?}", s.pieces[y-5]))
            },
            _ => ()
        };
        str.push_str("\n");
    }
    console_log!("{}", str);
}
