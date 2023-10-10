use std::time::Duration;
use std::thread;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::stats::Stats;
use crate::*;


pub fn run (args: crate::Args) {

    println!("{BLD}=== Sandbox Run ==={RST}");
    println!("threads: {}", args.threads);
    println!("iters:   {}", args.iters);
    println!("pps:     {}", args.pps);

    let mut stats = Stats::new();
    let mut state = quaternion::SimState::new();
    //let mut rng   = ChaCha8Rng::seed_from_u64(2);
    let bot       = quaternion::Quaternion::with_threads(args.threads);
    
    println!("init state:\n{}", state);
    bot.advance(&state);
    bot.start();

    for _ in 0..args.iters {
        thread::sleep(Duration::from_millis((1000.0 / args.pps) as u64));

        // Get solution & stats
        let mov = bot.solution();
        let (n_state, move_stats) = state.advance(&mov);
        stats.accumulate(&move_stats, &bot.stats());
        state = n_state;

        // Advance
        bot.advance(&state);

        // Refresh bag
        state.draw();

        // Render
        println!("{state}");
        println!("{:?}", move_stats);
    }
    bot.stop();
    println!("{}", stats);
}

