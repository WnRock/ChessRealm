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
