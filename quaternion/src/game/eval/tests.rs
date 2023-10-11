use super::*;

#[test]
fn eval1 () {
    let a = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold: none
. . . . . . . . . .  queue:
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
. . . . . . . . . .
# # . . . . . . . .
# . . . . # # . . .
# # . # # # # # # #
    ";
    let a_stats = MoveStats { attacks: 0, ds: 0, tspin: false };
    let b = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  none
. . . . . . . . . .  queue:
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
. . . . . # # # . .
. . . . . # # # . .
. # # # # # # # # #
# # . # # # # # # #
    ";
    let b_stats = MoveStats { attacks: 0, ds: 0, tspin: false };

    assert!(evaluate(&State::from_str(&a), a_stats, Mode::Norm) > evaluate(&State::from_str(&b), b_stats, Mode::Norm));
}

#[test]
fn eval2 () {
    let a = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold: S 
. . . . . . . . . .  queue:
. . . . . . . . . .  L
. . . . . . . . . .  J
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  S
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . # . . .
. . . # . # # . . .
. . # # # # # # # .
# . # # # # # # # #
# . # # # # # # # #
# . # # # # # # # #
    ";
    let a_stats = MoveStats { attacks: 4, ds: 2, tspin: true };
    let b = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold: S 
. . . . . . . . . .  queue:
. . . . . . . . . .  L
. . . . . . . . . .  J
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  S
. . . . . . . . . .
. . . . . . . . . .
. . . . . . # . . #
. . . # . # # . # #
. . # # # # # # # #
. . . # # # # # # #
# . # # # # # # # #
# . # # # # # # # #
# . # # # # # # # #
# . # # # # # # # #
    ";
    let b_stats = MoveStats { attacks: 0, ds: 0, tspin: false };

    assert!(evaluate(&State::from_str(&a), a_stats, Mode::Norm) > evaluate(&State::from_str(&b), b_stats, Mode::Norm));
}

#[test]
fn eval3 () {
    let a = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  T
. . . . . . . . . .  queue:
. . . . . . . . . .  L
. . . . . . . . . .  J
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  T
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
# . . . . . . . . .
# . . . . # . . . .
# . . . . # # # # #
# # # . . . # # # #
# # # # . # # # # #
";
    let a_stats = MoveStats { attacks: 0, ds: 0, tspin: false };
    let b = "
. . . . . . . . . .  b2b:    0
. . . . . . . . . .  combo:  0
. . . . . . . . . .
. . . . . . . . . .  hold:  T
. . . . . . . . . .  queue:
. . . . . . . . . .  L
. . . . . . . . . .  J
. . . . . . . . . .  O
. . . . . . . . . .  I
. . . . . . . . . .  T
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
# . . . . . . . . .
# . . . . . . . . .
# . . . . . # # # #
# # # . . # # # # #
# # # # . # # # # #
    ";
    let b_stats = MoveStats { attacks: 0, ds: 0, tspin: false };

    assert!(evaluate(&State::from_str(&a), a_stats, Mode::Norm) > evaluate(&State::from_str(&b), b_stats, Mode::Norm));
}
