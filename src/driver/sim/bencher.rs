use std::time::Instant;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;


use tetron::{solve, State, EvaluatorMode, config};

use crate::colors::*;

pub fn run () {
    // Tests
    // - Upstack (sandbox)  TODO
    // - Downstack (cheese) TODO
    // - Counterattack (backfire) TODO
    //
    {
        let mut state = State::new();
        let mut bag = vec![];
        let mut rng   = ChaCha8Rng::seed_from_u64(2);
        const ITERS: usize = 100;
        
        let mut atks: u32 = 0;
        let mut dt = 0.0;
        
        while state.pieces.len() < 6 {
            state.pieces.push_back(super::draw(&mut rng, &mut bag));
        }
        for _ in 0..ITERS {
            // Draw pieces
            while state.pieces.len() < 6 {
                state.pieces.push_back(super::draw(&mut rng, &mut bag));
            }

            // Solve & Bench
            tetron::bench_increment_solve();
            let start = Instant::now();        
            if let Some(out) = solve(&state, &config::Config::new(3, EvaluatorMode::Norm)) {
                let elapsed = start.elapsed().as_micros();
                dt += elapsed as f64 / 1_000_000.0;
                atks += out.0.props.atk as u32;

                state.field = out.0.field;
                state.pieces = out.0.pieces;
                state.hold = out.0.hold;
            } else {
                println!("{BLD}No results found, game over.{RST}");
                break;
            }
        }
        println!("pieces    : {}"   , ITERS);
        println!("attacks   : {}"   , atks);
        println!("app       : {:.2}", atks as f64 / ITERS as f64);
        println!("apm       : {:.2}", atks as f64 / (dt as f64 / 60.0));
        println!("pps       : {:.2}", (ITERS as f64 / dt as f64));

        tetron::print_bench_result();
    }
 
}
