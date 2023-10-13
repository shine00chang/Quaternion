mod sim;

use clap::{Parser, ValueEnum};


pub const RST: &str = "\x1b[0m";
pub const BLD: &str = "\x1b[1m";
pub const HLT: &str = "\x1b[48;5;226m";


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(short, long, default_value_t = 100)]
    iters: u32,

    #[arg(short, long, default_value_t = 3.0)]
    pps: f32,
    
    #[arg(short, long, default_value_t = 8)]
    threads: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Sandbox,
    Backfire
//    Cheese,
}

fn main() {
    let args = Args::parse();
    
    println!("arguments: {:?}\n", args);

    match args.mode {
        Mode::Sandbox  => sim::sandbox::run(args),
        Mode::Backfire => sim::backfire::run(args),
//        Mode::Cheese   => sim::cheese::run(args),
        _ => println!("Not yet implemented")
    }
}
