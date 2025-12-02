use crate::game::piece::{Piece, PieceKind, PieceSide};

pub type BoardState = [[Option<Piece>; 9]; 10];

pub fn init_board() -> BoardState {
    let mut board: BoardState = [[None; 9]; 10];

    let setup_row = |row: usize, side: PieceSide, board: &mut BoardState| {
        let pieces: [PieceKind; 9] = [
            PieceKind::Ju,
            PieceKind::Ma,
            PieceKind::Xiang,
            PieceKind::Shi,
            PieceKind::Jiang,
            PieceKind::Shi,
            PieceKind::Xiang,
            PieceKind::Ma,
            PieceKind::Ju,
        ];
        for (col, &pt) in pieces.iter().enumerate() {
            board[row][col] = Some(Piece { side, kind: pt });
        }
    };

    setup_row(0, PieceSide::Black, &mut board);
    board[2][1] = Some(Piece {
        side: PieceSide::Black,
        kind: PieceKind::Pao,
    });
    board[2][7] = Some(Piece {
        side: PieceSide::Black,
        kind: PieceKind::Pao,
    });
    for i in 0..5 {
        board[3][i * 2] = Some(Piece {
            side: PieceSide::Black,
            kind: PieceKind::Zu,
        });
    }

    setup_row(9, PieceSide::Red, &mut board);
    board[7][1] = Some(Piece {
        side: PieceSide::Red,
        kind: PieceKind::Pao,
    });
    board[7][7] = Some(Piece {
        side: PieceSide::Red,
        kind: PieceKind::Pao,
    });
    for i in 0..5 {
        board[6][i * 2] = Some(Piece {
            side: PieceSide::Red,
            kind: PieceKind::Zu,
        });
    }

    board
}
