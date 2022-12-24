use crate::{
    board::Board,
    enums::{PieceType, PieceTypeColor},
};
use std::str::FromStr;

pub fn load_board(board: &mut Board, s: &str) {
    let r = board.rows;
    let c = board.cols;

    if r * c < s.len() as isize {
        panic!(
            "Invalid board string, expected len <= {}, found {}",
            r * c,
            s.len()
        );
    }

    for (i, ch) in s.chars().enumerate() {
        let i = i as isize;
        let row = i / c;
        let col = i % c;
        match ch {
            ' ' => board.clear(row, col),
            '*' => board.set(row, col, PieceTypeColor::NotPlayable),
            _ => PieceType::from_str(&String::from(ch))
                .map(|pt| board.set(row, col, PieceTypeColor::Playable(pt)))
                .unwrap(),
        }
    }
}
