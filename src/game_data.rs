use rust_tetris_core::{
    board::Board, enums::PieceType, holder::HoldPiece, piece_bag::PieceBag, pieces::Piece,
};

pub struct GameData {
    pub board: Board,
    pub piece_bag: PieceBag,
    pub hold_piece: Option<HoldPiece>,

    pub lines: u32,
    pub time: f32,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            board: Board::new(20, 10),
            piece_bag: PieceBag::new(6, None),
            hold_piece: Some(HoldPiece {
                piece: Piece::new(PieceType::S),
                already_hold: false,
            }),
            lines: 0,
            time: 0.,
        }
    }
}
