use std::time::Duration;
use std::thread;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use quaternion::game;
use super::state;
use super::stats::Stats;
use crate::colors::*;


pub fn run (args: crate::Args) {

    println!("{BLD}=== Sandbox Run ==={RST}");
    println!("threads: {}", args.threads);
    println!("iters:   {}", args.iters);
    println!("pps:     {}", args.pps);

    let mut stats = Stats::new();
    let mut state = state::State::new(game::State::new());
    let mut rng   = ChaCha8Rng::seed_from_u64(2);
    let bot       = quaternion::Quaternion::with_threads(args.threads);
    
    let mut cheese_clears = 0;
    let mut cheese_rows = 0;
    let cheese_lines = args.iters;
    while cheese_rows < 10.min(cheese_lines) {
        state.gen_garbage(&mut rng, 1);
        cheese_rows += 1;
    }   

    state.draw(&mut rng);

    println!("init state:\n{}", state);
    bot.advance(state.get_state().clone());
    bot.start();

    while cheese_lines > cheese_clears {
        thread::sleep(Duration::from_millis((1000.0 / args.pps) as u64));

        let (mv, game_state) = bot.solution();
        let bot_stats = bot.stats();
        state.advance(&mv, &game_state);
        state.draw(&mut rng);

        // Count & make cheese
        for y in (20-cheese_rows)..20 {
            if game_state.props.clears & 1 << y > 0 {
                cheese_rows -= 1;
                cheese_clears += 1;
            }
        }
    
        if game_state.props.clears == 0 {
           while cheese_rows + cheese_clears < cheese_lines && cheese_rows < 10 {
                state.gen_garbage(&mut rng, 1);
                cheese_rows += 1;
            }
        }

        bot.advance(state.get_state().clone());

        stats.accumulate(&game_state, &bot_stats);
        println!("{}", state);
    }
    bot.stop();
    println!("{}", stats);
}

