use std::thread;
use std::time::Duration;

#[test]
pub fn tspin () {

    let input = "
    . . . . . . . . . .  b2b:    0
    . . . . . . . . . .  combo:  0
    . . . . . . . . . .
    . . . . . . . . . .  hold:  None
    . . . . . . . . . .  queue:
    . . . . . . . . . .  T
    . . . . . . . . . .  S
    . . . . . . . . . .  L
    . . . . . . . . . .  J
    . . . . . . . . . .  O
    . . . . . . . . . . 
    . . . . . . . . . .
    . . . . . . . . . . 
    . . . . . . . . . .
    . . . . . . . . . . 
    . . . . . . . . . . 
    . . . . . . . . . . 
    # . . . . . . . . .
    . . . # # # # # # #
    # . # # # # # # # #
    ";

    let state = quaternion::SimState::from_str(input);
    let bot = quaternion::Quaternion::with_threads(1);
    
    println!("init state:\n{}", state);
    bot.advance(&state);
    bot.start();

    thread::sleep(Duration::from_millis(1000 as u64));

    // Get solution & stats
    let mov = bot.solution();
    let (state, stats) = state.advance(&mov);

    bot.stop();

    // Render
    println!("{state}");

    assert_eq!(stats.tspin, true);
    assert_eq!(stats.attacks, 4);
}

