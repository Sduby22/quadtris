use rust_tetris_core::{
    board::Board,
    enums::PieceType,
    holder::HoldPiece,
    piece_bag::PieceBag,
    pieces::{Piece, PieceWithPosition},
};

use macroquad::prelude::*;

pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct KeyBind {
    pub left: KeyCode,
    pub right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,

    pub rotate_cw: KeyCode,
    pub rotate_ccw: KeyCode,
    pub hold: KeyCode,

    pub restart: KeyCode,
    pub escape: KeyCode,
}

impl Default for KeyBind {
    fn default() -> Self {
        Self {
            left: KeyCode::A,
            right: KeyCode::D,
            soft_drop: KeyCode::S,
            hard_drop: KeyCode::W,
            rotate_cw: KeyCode::K,
            rotate_ccw: KeyCode::J,
            hold: KeyCode::L,
            restart: KeyCode::R,
            escape: KeyCode::Escape,
        }
    }
}

pub struct GameData {
    pub keybind: KeyBind,
    pub board: Board,
    pub piece_bag: Option<PieceBag>,
    pub hold_piece: Option<HoldPiece>,
    pub curr_piece: Option<PieceWithPosition>,

    pub gravity: f32,
    pub das: usize,
    pub arr: usize,
    pub soft_drop_gravity: f32,
    pub freeze_delay: usize,

    pub freeze_left: usize,
    pub accumulated_down: f32,
    pub accumulated_move: f32,

    pub state: GameState,
    pub lines: u32,
    pub time: f32,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            keybind: KeyBind::default(),
            board: Board::new(40, 10),
            piece_bag: None,
            hold_piece: None,
            curr_piece: None,

            gravity: 0.0156,
            das: 9,
            arr: 2,
            freeze_delay: 30,
            soft_drop_gravity: 20.,

            freeze_left: 30,
            accumulated_down: 0.,
            accumulated_move: 0.,

            state: GameState::Playing,
            lines: 0,
            time: 0.,
        }
    }

    pub fn clear(&mut self) {
        self.board.clear_all();
        self.piece_bag = None;
        self.hold_piece = None;
        self.curr_piece = None;
        self.lines = 0;
        self.time = 0.;
    }

    pub fn start(&mut self) {
        self.state = GameState::Playing;
        self.piece_bag = Some(PieceBag::new(6, None));
        self.gravity = 0.0156;
    }
}
