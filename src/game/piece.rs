use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PieceSide {
    Red,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PieceKind {
    Jiang,
    Shi,
    Xiang,
    Ma,
    Ju,
    Pao,
    Zu,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub side: PieceSide,
    pub kind: PieceKind,
}

impl Piece {
    /// Returns the Chinese character label for this piece.
    pub fn label(&self) -> &'static str {
        match self.side {
            PieceSide::Red => match self.kind {
                PieceKind::Jiang => "帅",
                PieceKind::Shi => "仕",
                PieceKind::Xiang => "相",
                PieceKind::Ma => "马",
                PieceKind::Ju => "车",
                PieceKind::Pao => "炮",
                PieceKind::Zu => "兵",
            },
            PieceSide::Black => match self.kind {
                PieceKind::Jiang => "将",
                PieceKind::Shi => "士",
                PieceKind::Xiang => "象",
                PieceKind::Ma => "马",
                PieceKind::Ju => "车",
                PieceKind::Pao => "炮",
                PieceKind::Zu => "卒",
            },
        }
    }
}
