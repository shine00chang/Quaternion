use super::*;

const TEST_BOARDS: Vec<Board> = vec![
    // BLANK
    Board {
        v: [
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
        ]
    },
];

#[test]
fn shift_l () {

    // SHIFT RIGHT FROM SPAWN
    let piece = Piece::T;
    let board = &TEST_BOARDS[0];
    let conflict_table = ConflictTable::from(board, piece);
    let mov = Move {
        x: 4,
        y: 19,
        r: Rotation::N,
        list: 0,
    };
    let mov = mov.shift(1, &conflict_table).unwrap();
    assert_eq!(mov.x, 5);
    assert_eq!(mov.parse_list(), vec![Key::R]);

    // LEFT TUCK
    let piece = Piece::J;
    let board = Board {
        v: [
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0100,
            0b0000_0000_0000_0000_0111,
        ]
    };
    let conflict_table = ConflictTable::from(&board, piece);
    let mov = Move {
        x: 3,
        y: 0,
        r: Rotation::N,
        list: 0,
    }; 
    let mov = mov.shift(-1, &conflict_table).unwrap();
    assert_eq!(mov.x, 2);
    assert_eq!(mov.parse_list(), vec![Key::L]);
}


#[test]
fn rotate () {

    // L Kick on wall
    let piece = Piece::L;
    let board = &TEST_BOARDS[0];
    let conflict_table = ConflictTable::from(board, piece);
    let mov = Move {
        x: 8,
        y: 19,
        r: Rotation::N,
        list: 0,
    };
    let mov = mov.drop(board, piece).unwrap();
    let mov = mov.ccw(&conflict_table).unwrap();
    assert_eq!(mov.x, 9);
    assert_eq!(mov.y, 18);
    assert_eq!(mov.r, Rotation::W);
    assert_eq!(mov.parse_list(), vec![Key::Drop, Key::CCW]);

    // TST 
    let piece = Piece::T;
    let board = Board {
        v: [
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0010,
            0b0000_0000_0000_0000_0101,
            0b0000_0000_0000_0001_0000,
            0b0000_0000_0000_0001_1111,
        ]
    };
    let conflict_table = ConflictTable::from(&board, piece);
    let mov = Move {
        x: 3,
        y: 0,
        r: Rotation::N,
        list: 0,
    }; 
    let mov = mov.drop(&board, piece).unwrap();
    let mov = mov.shift(-1, &conflict_table).unwrap();
    let mov = mov.cw(&conflict_table).unwrap();
    assert_eq!(mov.x, 1);
    assert_eq!(mov.y, 1);
    assert_eq!(mov.r, Rotation::E);
    assert_eq!(mov.parse_list(), vec![Key::Drop, Key::L, Key::CW]);
}

#[test]
fn drop () {

    // DROP INTO CAVITY 
    let piece = Piece::T;
    let board = Board {
        v: [
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_1000,
            0b0000_0000_0000_0000_0100,
            0b0000_0000_0000_0000_1000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
        ]
    };
    let conflict_table = ConflictTable::from(&board, piece);
    let mov = Move {
        x: 4,
        y: 19,
        r: Rotation::N,
        list: 0,
    };
    let mov = mov.drop(&board, piece).unwrap();
    assert_eq!(mov.y, 4);
    assert_eq!(mov.parse_list(), vec![Key::Drop]);

    // CLIP 
    let piece = Piece::T;
    let board = Board {
        v: [
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0100,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
        ]
    };
    let conflict_table = ConflictTable::from(&board, piece);
    let mov = Move {
        x: 4,
        y: 19,
        r: Rotation::N,
        list: 0,
    };
    let mov = mov.drop(&board, piece).unwrap();
    assert_eq!(mov.y, 3);
    assert_eq!(mov.parse_list(), vec![Key::Drop]);
}