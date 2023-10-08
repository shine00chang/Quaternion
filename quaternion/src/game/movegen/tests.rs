use super::*;

const TEST_BOARDS: [Board; 1] = [
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

impl std::fmt::Display for ConflictTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..20).rev() {
            for x in 0..10 {
                let b = self.v[2][x] & (1 << y) != 0;
                write!(f, "{} ", if b { 'x' } else { '.' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

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
    assert_eq!(mov.r, Rotation::W);
    assert_eq!(mov.x, 9);
    assert_eq!(mov.y, 1);
    assert_eq!(mov.parse_list(), vec![Key::Drop, Key::CCW]);

    // TST 
    let piece = Piece::T;
    let board = Board {
        v: [
            0b0000_0000_0000_0001_1111,
            0b0000_0000_0000_0001_0000,
            0b0000_0000_0000_0000_0101,
            0b0000_0000_0000_0000_0010,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
        ]
    };
    let conflict_table = ConflictTable::from(&board, piece);
    let mov = Move {
        x: 3,
        y: 19,
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
            0b0000_0000_0000_0000_0100,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000,
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



#[test]
fn held () {
    let mut mov = Move::default();
    assert!(!mov.held());

    mov.add_key(&Key::Hold);
    assert!(mov.held());
}


#[test]
fn list_space_limit () {
    let mut mov = Move::default();
    for _ in 0..19 {
        mov.add_key(&Key::L);
    }
    assert!(!mov.list_has_space());
}


#[test]
fn movegen () {
    let board = &TEST_BOARDS[0]; 

    let state = State {
        board: board.clone(),
        queue: vec![Piece::T].into_iter().collect(),
        b2b: 0,
        combo: 0,
        hold: None,
    };

    let moves = gen_moves(&state);

    for mov in moves {
        let state = state.clone().apply_move(&mov);
        println!("{state}");
    }
}
