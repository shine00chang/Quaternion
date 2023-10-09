use super::*;


#[test]
fn clears () {
    let input = "
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
    . # # # # . . . . .
    . # # # # . . . . .
    # # # # # # . . . .
    # # . . . # . # # .
    # # # # # # # # # #
    . . # . # # . . # #
    # # # # # # # # # #
    # # # # # # # # # #
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
    . # # # # . . . . .
    . # # # # . . . . .
    # # # # # # . . . .
    # # . . . # . # # .
    . . # . # # . . # #
    ";

    let mut input = make_board(input);
    let output = make_board(output);
    input.clear();
    
    println!("> expected:\n{output}");
    println!("> got:\n{input}");
    assert_eq!(input, output);
}

fn make_board (s: &str) -> Board {
    let mut b = [[false; 10]; 20];
    let mut y = 0;
    let mut x = 0;
    for ch in s.chars() {
        if ch == '#' || ch == '.' {
            b[y][x] = ch == '#'; 
            x += 1;
            if x == 10 {
                x = 0;
                y += 1;
            }
            if y == 20 {
                break;
            }
        }
    }

    let mut v = [0; 10];
    for x in 0..10 {
        for y in 0..20 {
            if b[19-y][x] {
                v[x] |= 1 << y;
            }
        }
    }

    Board { v }
}
