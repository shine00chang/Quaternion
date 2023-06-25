use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use web_sys::console;

/*
macro_rules! console_log {
    ($($arg: expr), *) => {   
        console::log_1(
            &JsValue::from_str(
                &format!(
                    $( $arg, )*
                )
            )
        );  
    }
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
    config: tetron::config::Config,
}

fn to_tetron (p: Piece) -> tetron::Piece {
    match p {
        Piece::T => tetron::Piece::T,
        Piece::I => tetron::Piece::I,
        Piece::O => tetron::Piece::O,
        Piece::J => tetron::Piece::J,
        Piece::L => tetron::Piece::L,
        Piece::S => tetron::Piece::S,
        Piece::Z => tetron::Piece::Z,
        Piece::Some => tetron::Piece::O, // Inaccuracy, but should not have repercussions
        Piece::None => tetron::Piece::None,
    }
}

#[wasm_bindgen]
impl Input {
    #[wasm_bindgen]
    pub fn new () -> Self {
        Self {
            board:  vec![Piece::None; 200],
            pieces: vec![Piece::None; 6],
            hold:   Piece::None,
            config: tetron::config::Config::new(2, tetron::EvaluatorMode::Norm)
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

    #[wasm_bindgen]
    pub fn set_depth(&mut self, d: usize) {
       self.config.depth = d as u8; 
    }

    fn parse (&self) -> tetron::State {
        // Set Board
        let mut state = tetron::State::new();
        
        for y in 0..20 {
            for x in 0..10 {
                if self.board[y*10 +x] != Piece::None {
                    state.field.m[y] += 1 << x;
                }
            }
        }

        // Set Piece
        for p in &self.pieces {
            state.pieces.push_back(to_tetron(*p));
        }

        // Set Hold
        state.hold = to_tetron(self.hold);

        console_log!("Input State (wasm-driver):\n{}", state);
        state
    }

    #[wasm_bindgen]
    pub fn run (self) -> Output {
        let state: tetron::State = self.parse(); 
        let result: Option<(tetron::State, tetron::Move, f32)> = tetron::solve(&state, &self.config);

        if let Some(result) = result {
            // == Debug logging == 
            print_state(result.0);
            // ====
            
            Output::new(result.1)
        } else {
            console_log!("No result"); 
            Output::none()
        }
    }
    

    // Testing Purposes
    #[wasm_bindgen] 
    pub fn test (&self, p: Piece) {
        console_log!("== Test ==: Hello From Rust!!");
        console_log!("== Test ==: Piece: {:?}", p);
    }
}

#[wasm_bindgen]
pub struct Output {
    list: VecDeque<Key>
}

impl Output {
    pub fn new (mov: tetron::Move) -> Self {
        let mut list: VecDeque<Key> = VecDeque::new();
        mov.parse_list().iter().for_each(|k| match k {
            tetron::Key::Left       => list.push_back(Key::Left), 
            tetron::Key::Right      => list.push_back(Key::Right),
            tetron::Key::DASLeft    => for _ in 0..5 { list.push_back(Key::Left) },
            tetron::Key::DASRight   => for _ in 0..5 { list.push_back(Key::Left) },
            tetron::Key::Cw         => list.push_back(Key::Cw),
            tetron::Key::Ccw        => list.push_back(Key::Ccw),
            tetron::Key::_180       => list.push_back(Key::_180),
            tetron::Key::HardDrop   => list.push_back(Key::HardDrop),
            tetron::Key::SoftDrop   => list.push_back(Key::SoftDrop),
            tetron::Key::Hold       => list.push_back(Key::Hold)
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


fn print_state (s: tetron::State) {
   
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
*/
