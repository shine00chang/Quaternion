use std::time::Duration;
use std::thread;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use quattron::game;
use super::state;
use super::stats::Stats;
use crate::colors::*;


pub fn run (args: crate::Args) {

    println!("{BLD}=== Sandbox Run ==={RST}");
    println!("threads: {}", args.threads);
    println!("iters:   {}", args.iters);
    println!("pps:     {}", args.pps);

    let mut stats = Stats::default();
    let mut state = state::State::new(game::State::new());
    let mut rng   = ChaCha8Rng::seed_from_u64(2);
    let bot       = quattron::Quattron::with_threads(args.threads);
    
    state.draw(&mut rng);

    println!("init state:\n{}", state);
    bot.advance(state.get_state().clone());
    bot.start();

    for _ in 0..args.iters {
        thread::sleep(Duration::from_millis((1000.0 / args.pps) as u64));

        let (mv, game_state) = bot.solution();
        state.advance(&mv, &game_state);
        state.draw(&mut rng);
        bot.advance(state.get_state().clone());

        stats.accumulate(&game_state);
        println!("{}", state);
    }
    bot.stop();
    println!("{}", stats);
}

