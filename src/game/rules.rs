use crate::game::board::BoardState;
use crate::game::piece::{Piece, PieceKind, PieceSide};

pub type Position = (usize, usize);

pub fn is_within_board(pos: Position) -> bool {
    pos.0 < 10 && pos.1 < 9
}

pub fn is_within_palace(pos: Position, side: PieceSide) -> bool {
    let (row, col) = pos;
    let in_cols = (3..=5).contains(&col);
    match side {
        PieceSide::Red => (7..=9).contains(&row) && in_cols,
        PieceSide::Black => (0..=2).contains(&row) && in_cols,
    }
}

pub fn is_own_side(pos: Position, side: PieceSide) -> bool {
    match side {
        PieceSide::Red => pos.0 >= 5,
        PieceSide::Black => pos.0 <= 4,
    }
}

pub fn has_crossed_river(pos: Position, side: PieceSide) -> bool {
    match side {
        PieceSide::Red => pos.0 < 5,
        PieceSide::Black => pos.0 > 4,
    }
}

pub fn count_pieces_between(board: &BoardState, from: Position, to: Position) -> Option<usize> {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if r1 == r2 {
        let (min_c, max_c) = if c1 < c2 { (c1, c2) } else { (c2, c1) };
        let count = ((min_c + 1)..max_c)
            .filter(|&c| board[r1][c].is_some())
            .count();
        Some(count)
    } else if c1 == c2 {
        let (min_r, max_r) = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        let count = ((min_r + 1)..max_r)
            .filter(|&r| board[r][c1].is_some())
            .count();
        Some(count)
    } else {
        None
    }
}

fn is_valid_jiang_move(_board: &BoardState, from: Position, to: Position, side: PieceSide) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if !is_within_palace(to, side) {
        return false;
    }
    let row_diff = (r1 as i32 - r2 as i32).abs();
    let col_diff = (c1 as i32 - c2 as i32).abs();

    (row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1)
}

fn is_valid_shi_move(_board: &BoardState, from: Position, to: Position, side: PieceSide) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if !is_within_palace(to, side) {
        return false;
    }
    let row_diff = (r1 as i32 - r2 as i32).abs();
    let col_diff = (c1 as i32 - c2 as i32).abs();

    row_diff == 1 && col_diff == 1
}

fn is_valid_xiang_move(board: &BoardState, from: Position, to: Position, side: PieceSide) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if !is_own_side(to, side) {
        return false;
    }

    let row_diff = (r1 as i32 - r2 as i32).abs();
    let col_diff = (c1 as i32 - c2 as i32).abs();
    if row_diff != 2 || col_diff != 2 {
        return false;
    }

    let eye_row = (r1 + r2) / 2;
    let eye_col = (c1 + c2) / 2;
    board[eye_row][eye_col].is_none()
}

fn is_valid_ma_move(board: &BoardState, from: Position, to: Position) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    let row_diff = (r1 as i32 - r2 as i32).abs();
    let col_diff = (c1 as i32 - c2 as i32).abs();
    if !((row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)) {
        return false;
    }

    let (leg_row, leg_col) = if row_diff == 2 {
        let leg_r = if r2 > r1 { r1 + 1 } else { r1 - 1 };
        (leg_r, c1)
    } else {
        let leg_c = if c2 > c1 { c1 + 1 } else { c1 - 1 };
        (r1, leg_c)
    };
    board[leg_row][leg_col].is_none()
}

fn is_valid_ju_move(board: &BoardState, from: Position, to: Position) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if r1 != r2 && c1 != c2 {
        return false;
    }

    count_pieces_between(board, from, to) == Some(0)
}

fn is_valid_pao_move(board: &BoardState, from: Position, to: Position) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if r1 != r2 && c1 != c2 {
        return false;
    }

    let pieces_between = match count_pieces_between(board, from, to) {
        Some(count) => count,
        None => return false,
    };

    let is_capture = board[to.0][to.1].is_some();

    if is_capture {
        pieces_between == 1
    } else {
        pieces_between == 0
    }
}

fn is_valid_zu_move(_board: &BoardState, from: Position, to: Position, side: PieceSide) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    let row_diff = r2 as i32 - r1 as i32;
    let col_diff = (c1 as i32 - c2 as i32).abs();
    let row_move = row_diff.abs();
    if row_move > 1 || col_diff > 1 || (row_move == 0 && col_diff == 0) {
        return false;
    }

    let forward = match side {
        PieceSide::Red => -1,
        PieceSide::Black => 1,
    };
    if row_diff == -forward {
        return false;
    }

    if has_crossed_river(from, side) {
        (row_diff == forward && col_diff == 0) || (row_diff == 0 && col_diff == 1)
    } else {
        row_diff == forward && col_diff == 0
    }
}

pub fn are_generals_facing(board: &BoardState) -> bool {
    let mut red_jiang: Option<Position> = None;
    let mut black_jiang: Option<Position> = None;

    for row in 7..=9 {
        for col in 3..=5 {
            if let Some(piece) = board[row][col] {
                if piece.kind == PieceKind::Jiang && piece.side == PieceSide::Red {
                    red_jiang = Some((row, col));
                }
            }
        }
    }

    for row in 0..=2 {
        for col in 3..=5 {
            if let Some(piece) = board[row][col] {
                if piece.kind == PieceKind::Jiang && piece.side == PieceSide::Black {
                    black_jiang = Some((row, col));
                }
            }
        }
    }

    if let (Some((r1, c1)), Some((r2, c2))) = (red_jiang, black_jiang) {
        if c1 == c2 {
            let (min_r, max_r) = (r2, r1);
            for row in (min_r + 1)..max_r {
                if board[row][c1].is_some() {
                    return false;
                }
            }
            return true;
        }
    }

    false
}

pub fn find_general(board: &BoardState, side: PieceSide) -> Option<Position> {
    let rows = match side {
        PieceSide::Red => 7..=9,
        PieceSide::Black => 0..=2,
    };

    for row in rows {
        for col in 3..=5 {
            if let Some(piece) = board[row][col] {
                if piece.kind == PieceKind::Jiang && piece.side == side {
                    return Some((row, col));
                }
            }
        }
    }
    None
}

pub fn is_under_attack(board: &BoardState, pos: Position, defender_side: PieceSide) -> bool {
    let attacker_side = match defender_side {
        PieceSide::Red => PieceSide::Black,
        PieceSide::Black => PieceSide::Red,
    };

    for row in 0..10 {
        for col in 0..9 {
            if let Some(piece) = board[row][col] {
                if piece.side == attacker_side {
                    if is_valid_piece_move(board, (row, col), pos, piece) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

pub fn is_in_check(board: &BoardState, side: PieceSide) -> bool {
    if let Some(general_pos) = find_general(board, side) {
        is_under_attack(board, general_pos, side)
    } else {
        false
    }
}

fn is_valid_piece_move(board: &BoardState, from: Position, to: Position, piece: Piece) -> bool {
    match piece.kind {
        PieceKind::Jiang => is_valid_jiang_move(board, from, to, piece.side),
        PieceKind::Shi => is_valid_shi_move(board, from, to, piece.side),
        PieceKind::Xiang => is_valid_xiang_move(board, from, to, piece.side),
        PieceKind::Ma => is_valid_ma_move(board, from, to),
        PieceKind::Ju => is_valid_ju_move(board, from, to),
        PieceKind::Pao => is_valid_pao_move(board, from, to),
        PieceKind::Zu => is_valid_zu_move(board, from, to, piece.side),
    }
}

pub fn is_valid_move(board: &BoardState, from: Position, to: Position, side: PieceSide) -> bool {
    if !is_within_board(from) || !is_within_board(to) {
        return false;
    }

    let piece = match board[from.0][from.1] {
        Some(p) => p,
        None => return false,
    };
    if piece.side != side {
        return false;
    }

    if let Some(target_piece) = board[to.0][to.1] {
        if target_piece.side == side {
            return false;
        }
    }

    if !is_valid_piece_move(board, from, to, piece) {
        return false;
    }

    let mut test_board = *board;
    test_board[to.0][to.1] = test_board[from.0][from.1];
    test_board[from.0][from.1] = None;

    if are_generals_facing(&test_board) {
        return false;
    }

    if is_in_check(&test_board, side) {
        return false;
    }

    true
}

pub fn get_valid_moves(board: &BoardState, from: Position, side: PieceSide) -> Vec<Position> {
    let mut valid_moves = Vec::new();

    if let Some(piece) = board[from.0][from.1] {
        if piece.side != side {
            return valid_moves;
        }
    } else {
        return valid_moves;
    }

    for row in 0..10 {
        for col in 0..9 {
            if is_valid_move(board, from, (row, col), side) {
                valid_moves.push((row, col));
            }
        }
    }

    valid_moves
}

pub fn get_all_valid_moves(board: &BoardState, side: PieceSide) -> Vec<(Position, Position)> {
    let mut all_moves = Vec::new();

    for row in 0..10 {
        for col in 0..9 {
            if let Some(piece) = board[row][col] {
                if piece.side == side {
                    let from = (row, col);
                    for to in get_valid_moves(board, from, side) {
                        all_moves.push((from, to));
                    }
                }
            }
        }
    }

    all_moves
}

pub fn is_checkmate(board: &BoardState, side: PieceSide) -> bool {
    is_in_check(board, side) && get_all_valid_moves(board, side).is_empty()
}

pub fn is_stalemate(board: &BoardState, side: PieceSide) -> bool {
    !is_in_check(board, side) && get_all_valid_moves(board, side).is_empty()
}
