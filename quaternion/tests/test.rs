use std::thread;
use std::time::Duration;
use quaternion::*;

fn iter (mut state: SimState, iters: u32) -> SimState {
    let bot = Quaternion::with_threads(8);
    
    println!("init state:\n{}", state);
    bot.advance(&state);
    bot.start();

    for _ in 0..iters {
        thread::sleep(Duration::from_millis(2000));

        // Get solution & stats
        let mov = bot.solution();
        (state, _) = state.advance(&mov);
        bot.advance(&state);

        println!("{state}");
    }
    bot.stop();

    state
}

#[test]
fn tspin1 () {
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

    let output = "
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . . 
. . . . . . . . . .
. . . . . . . . . . 
. . . . . . . . . .
. . . . . . . . . . 
. . . . . . . . . . 
. . . . . . . . . . 
. . . . . . . . . . 
. . . . . . . . . . 
# . . . . . . . . .
    ";

    let state = iter(SimState::from_str(input), 1);

    // Render
    println!("expected:\n{output}");
    println!("got:\n{state}");

    assert!(state.equals(output));
}

#[test]
fn tspin2 () {

    let input = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold: T 
. . . . . . . . . .  queue:
. . . . . . . . . .  Z
. . . . . . . . . .  L
. . . . . . . . . .  T
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
# . . . . . # # # #
# # . # # # # # # #
    ";

    let output = "
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . . 
. . . . . . . . . .
. . . . . . . . . . 
. . . . . . . . . .
. . . . . . . . . . 
. . . . . . . . . . 
. . . . . . . . . . 
. . . . . . . . . . 
. . . . . . . . . . 
# . . # # . . . . .
    ";

    let state = iter(SimState::from_str(input), 3);

    // Render
    println!("expected:\n{output}");
    println!("got:\n{state}");

    assert!(state.equals(output));
}


#[test]
fn tspin3 () {

    let input = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  1
. . . . . . . . . .
. . . . . . . . . .  hold:  Some(T)
. . . . . . . . . .  queue:
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  T
. . . . . . . . . .  Z
. . . . . . . . . .  S
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . # . . .
. . # # . # # . # .
. . . # # # # # # #
# . # # # # # # # #
    ";

    let output = "
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .  
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . # . . .
. . # # . # # . # .
";

    let state = iter(SimState::from_str(input), 3);

    // Render
    println!("expected:\n{output}");
    println!("got:\n{state}");

    assert!(state.equals(output));
}

#[test]
fn tspin4 () {

    let input = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  T
. . . . . . . . . .  queue:
. . . . . . . . . .  J
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  T
. . . . . . . . . .  Z
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . # # . . .
. # # # # # # # # #
# # . # # # # # # #
    ";

    let output = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  T
. . . . . . . . . .  queue:
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  T
. . . . . . . . . .  Z
. . . . . . . . . .  J
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
# # . . . . . . . .
# . . . . # # . . .
# # . # # # # # # #
    ";

    let state = iter(SimState::from_str(input), 1);

    // Render
    println!("expected:\n{output}");
    println!("got:\n{state}");

    assert!(state.equals(output));
}

#[test]
fn tspin5 () {

    let input = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  Z
. . . . . . . . . .  queue:
. . . . . . . . . .  L
. . . . . . . . . .  J
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . #  T
. . . . . . . . . #
. . . . . . . . . #
. . . . . . . . . #
. . . # # # # . . #
. . # # # # # . # #
. # # # # # # . # #
. # # # # # # # # #
. # # # # # # # # #
. # # # # # # # # #
# # # # # # # . # #
    ";

    let output = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  L
. . . . . . . . . .  queue:
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  T
. . . . . . . . . . 
. . . . . . . . . # 
. . . . . . . . . #
. . . . . . . . . #
. . # . . . . . . #
. # # # # # # # # #
. # # # # # # # # #
. # # # # # # # # #
. # # # # # # # # #
. # # # # # # # # #
. # # # # # # # # #
# # # # # # # . # #
    ";

    let state = iter(SimState::from_str(input), 2);

    // Render
    println!("expected:\n{output}");
    println!("got:\n{state}");

    assert!(state.equals(output));
}
