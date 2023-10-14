use std::collections::VecDeque;

use wasm_bindgen::prelude::*;

use quaternion::Piece as QPiece;

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
    None = 0,
}

impl Piece {
    fn to_bot (self) -> QPiece {
        match self {
            Piece::T => QPiece::T,
            Piece::I => QPiece::I,
            Piece::L => QPiece::L,
            Piece::J => QPiece::J,
            Piece::S => QPiece::S,
            Piece::Z => QPiece::Z,
            Piece::O => QPiece::O,
            Piece::None => QPiece::None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Key {
    L    = 1,
    R    = 2,
    CW   = 3,
    CCW  = 4,
    Drop = 5,
    Hold = 6,
    HardDrop = 0, // Symbolizes end of list
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Input {
    board: [[bool; 10]; 20],
    pieces: [QPiece; 6],
    hold: Option<QPiece>,
}

#[wasm_bindgen]
impl Input {
    #[wasm_bindgen]
    pub fn new () -> Self {
        Self {
            board:  [[false; 10]; 20],
            pieces: [QPiece::None; 6],
            hold:   None,
        }
    }

    #[wasm_bindgen]
    pub fn set_board(&mut self, x: usize, y: usize) {
        assert!(x < 10 && y < 20);
        self.board[y][x] = true;
    }

    #[wasm_bindgen]
    pub fn set_pieces(&mut self, i: usize, p: Piece) {
        assert!(i < 6);
        self.pieces[i] = p.to_bot();
    }

    #[wasm_bindgen]
    pub fn set_hold(&mut self, p: Piece) {
        if p == Piece::None {
            self.hold = None;
        } else {
            self.hold = Some(p.to_bot());
        }
    }

    fn parse (self) -> quaternion::State {
        let state = quaternion::State::from_js(self.board, self.pieces, self.hold);
        console_log!("{state}");

        state
    }
}

#[wasm_bindgen]
pub struct Wrapper {
    bot: quaternion::Quaternion,
    state: Option<quaternion::State>,
}

#[wasm_bindgen]
impl Wrapper {
    #[wasm_bindgen]
    pub fn new (threads: u32) -> Self {
        Self {
            bot: quaternion::Quaternion::single(),
            state: None
        }
    }

    #[wasm_bindgen]
    pub fn run (&mut self, ms: u32) -> Output {
        let mov = self.bot.wasm_run(ms);
        console_log!("{:?}", mov.parse_list());
        let state = self.state.clone().unwrap();
        let state = state.apply_move(&mov);
        console_log!("{}", state);

        Output::from(mov)
    }

    #[wasm_bindgen]
    pub fn advance (&mut self, input: Input) {
        self.state = Some( input.parse() );
        self.bot.advance(&self.state.clone().unwrap());
    }
}


#[wasm_bindgen]
pub struct Output {
    list: VecDeque<Key>
}

impl Output {
    pub fn from (mov: quaternion::Move) -> Self {
        let mut list: VecDeque<_> = mov
            .parse_list()
            .iter()
            .map(|k| match k {
                quaternion::Key::L    => Key::L,
                quaternion::Key::R    => Key::R,
                quaternion::Key::CW   => Key::CW,
                quaternion::Key::CCW  => Key::CCW,
                quaternion::Key::Drop => Key::Drop,
                quaternion::Key::Hold => Key::Hold
            })
            .collect();
        list.push_back(Key::HardDrop);
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
        self.list.pop_front().unwrap()
    }
}


use std::fmt::Write;
fn print_state (s: quaternion::State) {
    let mut f = String::new();
    write!(f, "{s}").unwrap();
    console_log!("{f}");
}
