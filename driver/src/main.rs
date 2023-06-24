mod sim;

use clap::{Parser, ValueEnum};

pub mod colors {
    pub const RST: &str = "\x1b[0m";
    pub const BLD: &str = "\x1b[1m";
    pub const HLT: &str = "\x1b[48;5;226m";
    macro_rules! piece_color {
        ($p: expr) => {
            match $p {
                game::Piece::None => "\x1b[47;1m", // white
                game::Piece::J => "\x1b[48;5;20m", // blue
                game::Piece::L => "\x1b[48;5;208m", // bright red / orange
                game::Piece::S => "\x1b[48;5;46m", // green
                game::Piece::Z => "\x1b[48;5;9m", // red
                game::Piece::T => "\x1b[45;1m", // magenta
                game::Piece::I => "\x1b[48;5;51m", // cyan
                game::Piece::O => "\x1b[48;5;226m", // yellow
            }
        };
    }
    pub(crate) use piece_color;
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(short, long, default_value_t = 100)]
    iters: u32,

    #[arg(short, long, default_value_t = 3.0)]
    pps: f32,
    
    #[arg(short, long, default_value_t = 10)]
    threads: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Sandbox,
    Cheese,
    Backfire
}

fn main() {
    let args = Args::parse();
    
    println!("arguments: {:?}\n", args);

    match args.mode {
        Mode::Sandbox  => sim::sandbox::run(args),
        Mode::Cheese   => sim::cheese::run(args),
        Mode::Backfire => sim::backfire::run(args),
        _ => println!("Not yet implemented")
    }
}
