pub use nanoserde::DeJsonErr;
use nanoserde::{DeJson, SerJson};
use rust_tetris_core::{
    board::Board, holder::HoldPiece, piece_bag::PieceBag, pieces::PieceWithPosition,
};

use macroquad::prelude::*;

use crate::constants::{ARR_DEFAULT, DAS_DEFAULT, SOFT_DROP_DEFAULT};

#[derive(PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Key(KeyCode);

impl Key {
    pub fn new(code: KeyCode) -> Self {
        Self(code)
    }

    pub fn code(&self) -> KeyCode {
        self.0
    }

    pub fn is_pressed(&self) -> bool {
        is_key_pressed(self.code())
    }

    pub fn is_down(&self) -> bool {
        is_key_down(self.code())
    }
}

#[derive(SerJson, DeJson, Clone)]
pub struct KeyBind {
    pub left: Key,
    pub right: Key,
    pub soft_drop: Key,
    pub hard_drop: Key,

    pub rotate_cw: Key,
    pub rotate_ccw: Key,
    pub hold: Key,

    pub restart: Key,
    pub escape: Key,
}

impl Default for KeyBind {
    fn default() -> Self {
        Self {
            left: Key(KeyCode::A),
            right: Key(KeyCode::D),
            soft_drop: Key(KeyCode::S),
            hard_drop: Key(KeyCode::W),
            rotate_cw: Key(KeyCode::K),
            rotate_ccw: Key(KeyCode::J),
            hold: Key(KeyCode::L),
            restart: Key(KeyCode::R),
            escape: Key(KeyCode::Escape),
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
    pub das: f32,
    pub das_left: f32,
    pub arr: f32,
    pub soft_drop_gravity: f32,
    pub freeze_delay: f32,

    pub freeze_left: f32,
    pub accumulated_down: f32,
    pub accumulated_move: f32,
    pub move_state: MoveState,

    pub state: GameState,
    pub lines: u32,
    pub time: f32,
}

pub enum MoveState {
    Left,
    Right,
    No,
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
            soft_drop_gravity: SOFT_DROP_DEFAULT,
            accumulated_down: 0.,

            freeze_delay: 120.,
            freeze_left: 120.,

            das: DAS_DEFAULT,
            das_left: DAS_DEFAULT,

            arr: ARR_DEFAULT,
            accumulated_move: 0.,
            move_state: MoveState::No,
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
        self.piece_bag = Some(PieceBag::new(6, Some((get_time() * 1000.) as u64)));
        self.gravity = 0.0156;
    }
}

/// Serialize

#[derive(SerJson, DeJson)]
pub struct UserSettings {
    pub keybind: Option<KeyBind>,
    pub arr: Option<f32>,
    pub das: Option<f32>,
    pub soft_drop_gravity: Option<f32>,
}

macro_rules! override_if_some {
    ($game_data:ident, $user_settings:ident, $field:ident) => {
        if let Some(value) = $user_settings.$field {
            $game_data.$field = value;
        }
    };
}

pub fn load_user_settings(game_data: &mut GameData) {
    let store = quad_storage::STORAGE.lock().unwrap();
    let json = store.get("user_settings").unwrap_or_default();

    if let Ok(user_settings) = UserSettings::deserialize_json(&json) {
        override_if_some!(game_data, user_settings, keybind);
        override_if_some!(game_data, user_settings, arr);
        override_if_some!(game_data, user_settings, das);
        override_if_some!(game_data, user_settings, soft_drop_gravity);
    }
}

pub fn save_user_settings(game_data: &GameData) {
    let mut store = quad_storage::STORAGE.lock().unwrap();
    store.set(
        "user_settings",
        &UserSettings {
            keybind: Some(game_data.keybind.clone()),
            arr: Some(game_data.arr),
            das: Some(game_data.das),
            soft_drop_gravity: Some(game_data.soft_drop_gravity),
        }
        .serialize_json(),
    );
}

impl SerJson for Key {
    fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
        SerKeyCode::from(self.0).ser_json(d, s)
    }
}

impl DeJson for Key {
    fn de_json(
        state: &mut nanoserde::DeJsonState,
        input: &mut std::str::Chars,
    ) -> Result<Self, nanoserde::DeJsonErr> {
        let code = SerKeyCode::de_json(state, input)?;
        Ok(Key(KeyCode::from(code)))
    }
}

#[derive(SerJson, DeJson)]
enum SerKeyCode {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}

impl From<KeyCode> for SerKeyCode {
    fn from(k: KeyCode) -> Self {
        match k {
            KeyCode::Space => SerKeyCode::Space,
            KeyCode::Apostrophe => SerKeyCode::Apostrophe,
            KeyCode::Comma => SerKeyCode::Comma,
            KeyCode::Minus => SerKeyCode::Minus,
            KeyCode::Period => SerKeyCode::Period,
            KeyCode::Slash => SerKeyCode::Slash,
            KeyCode::Key0 => SerKeyCode::Key0,
            KeyCode::Key1 => SerKeyCode::Key1,
            KeyCode::Key2 => SerKeyCode::Key2,
            KeyCode::Key3 => SerKeyCode::Key3,
            KeyCode::Key4 => SerKeyCode::Key4,
            KeyCode::Key5 => SerKeyCode::Key5,
            KeyCode::Key6 => SerKeyCode::Key6,
            KeyCode::Key7 => SerKeyCode::Key7,
            KeyCode::Key8 => SerKeyCode::Key8,
            KeyCode::Key9 => SerKeyCode::Key9,
            KeyCode::Semicolon => SerKeyCode::Semicolon,
            KeyCode::Equal => SerKeyCode::Equal,
            KeyCode::A => SerKeyCode::A,
            KeyCode::B => SerKeyCode::B,
            KeyCode::C => SerKeyCode::C,
            KeyCode::D => SerKeyCode::D,
            KeyCode::E => SerKeyCode::E,
            KeyCode::F => SerKeyCode::F,
            KeyCode::G => SerKeyCode::G,
            KeyCode::H => SerKeyCode::H,
            KeyCode::I => SerKeyCode::I,
            KeyCode::J => SerKeyCode::J,
            KeyCode::K => SerKeyCode::K,
            KeyCode::L => SerKeyCode::L,
            KeyCode::M => SerKeyCode::M,
            KeyCode::N => SerKeyCode::N,
            KeyCode::O => SerKeyCode::O,
            KeyCode::P => SerKeyCode::P,
            KeyCode::Q => SerKeyCode::Q,
            KeyCode::R => SerKeyCode::R,
            KeyCode::S => SerKeyCode::S,
            KeyCode::T => SerKeyCode::T,
            KeyCode::U => SerKeyCode::U,
            KeyCode::V => SerKeyCode::V,
            KeyCode::W => SerKeyCode::W,
            KeyCode::X => SerKeyCode::X,
            KeyCode::Y => SerKeyCode::Y,
            KeyCode::Z => SerKeyCode::Z,
            KeyCode::LeftBracket => SerKeyCode::LeftBracket,
            KeyCode::Backslash => SerKeyCode::Backslash,
            KeyCode::RightBracket => SerKeyCode::RightBracket,
            KeyCode::GraveAccent => SerKeyCode::GraveAccent,
            KeyCode::World1 => SerKeyCode::World1,
            KeyCode::World2 => SerKeyCode::World2,
            KeyCode::Escape => SerKeyCode::Escape,
            KeyCode::Enter => SerKeyCode::Enter,
            KeyCode::Tab => SerKeyCode::Tab,
            KeyCode::Backspace => SerKeyCode::Backspace,
            KeyCode::Insert => SerKeyCode::Insert,
            KeyCode::Delete => SerKeyCode::Delete,
            KeyCode::Right => SerKeyCode::Right,
            KeyCode::Left => SerKeyCode::Left,
            KeyCode::Down => SerKeyCode::Down,
            KeyCode::Up => SerKeyCode::Up,
            KeyCode::PageUp => SerKeyCode::PageUp,
            KeyCode::PageDown => SerKeyCode::PageDown,
            KeyCode::Home => SerKeyCode::Home,
            KeyCode::End => SerKeyCode::End,
            KeyCode::CapsLock => SerKeyCode::CapsLock,
            KeyCode::ScrollLock => SerKeyCode::ScrollLock,
            KeyCode::NumLock => SerKeyCode::NumLock,
            KeyCode::PrintScreen => SerKeyCode::PrintScreen,
            KeyCode::Pause => SerKeyCode::Pause,
            KeyCode::F1 => SerKeyCode::F1,
            KeyCode::F2 => SerKeyCode::F2,
            KeyCode::F3 => SerKeyCode::F3,
            KeyCode::F4 => SerKeyCode::F4,
            KeyCode::F5 => SerKeyCode::F5,
            KeyCode::F6 => SerKeyCode::F6,
            KeyCode::F7 => SerKeyCode::F7,
            KeyCode::F8 => SerKeyCode::F8,
            KeyCode::F9 => SerKeyCode::F9,
            KeyCode::F10 => SerKeyCode::F10,
            KeyCode::F11 => SerKeyCode::F11,
            KeyCode::F12 => SerKeyCode::F12,
            KeyCode::F13 => SerKeyCode::F13,
            KeyCode::F14 => SerKeyCode::F14,
            KeyCode::F15 => SerKeyCode::F15,
            KeyCode::F16 => SerKeyCode::F16,
            KeyCode::F17 => SerKeyCode::F17,
            KeyCode::F18 => SerKeyCode::F18,
            KeyCode::F19 => SerKeyCode::F19,
            KeyCode::F20 => SerKeyCode::F20,
            KeyCode::F21 => SerKeyCode::F21,
            KeyCode::F22 => SerKeyCode::F22,
            KeyCode::F23 => SerKeyCode::F23,
            KeyCode::F24 => SerKeyCode::F24,
            KeyCode::F25 => SerKeyCode::F25,
            KeyCode::Kp0 => SerKeyCode::Kp0,
            KeyCode::Kp1 => SerKeyCode::Kp1,
            KeyCode::Kp2 => SerKeyCode::Kp2,
            KeyCode::Kp3 => SerKeyCode::Kp3,
            KeyCode::Kp4 => SerKeyCode::Kp4,
            KeyCode::Kp5 => SerKeyCode::Kp5,
            KeyCode::Kp6 => SerKeyCode::Kp6,
            KeyCode::Kp7 => SerKeyCode::Kp7,
            KeyCode::Kp8 => SerKeyCode::Kp8,
            KeyCode::Kp9 => SerKeyCode::Kp9,
            KeyCode::KpDecimal => SerKeyCode::KpDecimal,
            KeyCode::KpDivide => SerKeyCode::KpDivide,
            KeyCode::KpMultiply => SerKeyCode::KpMultiply,
            KeyCode::KpSubtract => SerKeyCode::KpSubtract,
            KeyCode::KpAdd => SerKeyCode::KpAdd,
            KeyCode::KpEnter => SerKeyCode::KpEnter,
            KeyCode::KpEqual => SerKeyCode::KpEqual,
            KeyCode::LeftShift => SerKeyCode::LeftShift,
            KeyCode::LeftControl => SerKeyCode::LeftControl,
            KeyCode::LeftAlt => SerKeyCode::LeftAlt,
            KeyCode::LeftSuper => SerKeyCode::LeftSuper,
            KeyCode::RightShift => SerKeyCode::RightShift,
            KeyCode::RightControl => SerKeyCode::RightControl,
            KeyCode::RightAlt => SerKeyCode::RightAlt,
            KeyCode::RightSuper => SerKeyCode::RightSuper,
            KeyCode::Menu => SerKeyCode::Menu,
            KeyCode::Unknown => SerKeyCode::Unknown,
        }
    }
}

impl From<SerKeyCode> for KeyCode {
    fn from(k: SerKeyCode) -> Self {
        match k {
            SerKeyCode::Space => KeyCode::Space,
            SerKeyCode::Apostrophe => KeyCode::Apostrophe,
            SerKeyCode::Comma => KeyCode::Comma,
            SerKeyCode::Minus => KeyCode::Minus,
            SerKeyCode::Period => KeyCode::Period,
            SerKeyCode::Slash => KeyCode::Slash,
            SerKeyCode::Key0 => KeyCode::Key0,
            SerKeyCode::Key1 => KeyCode::Key1,
            SerKeyCode::Key2 => KeyCode::Key2,
            SerKeyCode::Key3 => KeyCode::Key3,
            SerKeyCode::Key4 => KeyCode::Key4,
            SerKeyCode::Key5 => KeyCode::Key5,
            SerKeyCode::Key6 => KeyCode::Key6,
            SerKeyCode::Key7 => KeyCode::Key7,
            SerKeyCode::Key8 => KeyCode::Key8,
            SerKeyCode::Key9 => KeyCode::Key9,
            SerKeyCode::Semicolon => KeyCode::Semicolon,
            SerKeyCode::Equal => KeyCode::Equal,
            SerKeyCode::A => KeyCode::A,
            SerKeyCode::B => KeyCode::B,
            SerKeyCode::C => KeyCode::C,
            SerKeyCode::D => KeyCode::D,
            SerKeyCode::E => KeyCode::E,
            SerKeyCode::F => KeyCode::F,
            SerKeyCode::G => KeyCode::G,
            SerKeyCode::H => KeyCode::H,
            SerKeyCode::I => KeyCode::I,
            SerKeyCode::J => KeyCode::J,
            SerKeyCode::K => KeyCode::K,
            SerKeyCode::L => KeyCode::L,
            SerKeyCode::M => KeyCode::M,
            SerKeyCode::N => KeyCode::N,
            SerKeyCode::O => KeyCode::O,
            SerKeyCode::P => KeyCode::P,
            SerKeyCode::Q => KeyCode::Q,
            SerKeyCode::R => KeyCode::R,
            SerKeyCode::S => KeyCode::S,
            SerKeyCode::T => KeyCode::T,
            SerKeyCode::U => KeyCode::U,
            SerKeyCode::V => KeyCode::V,
            SerKeyCode::W => KeyCode::W,
            SerKeyCode::X => KeyCode::X,
            SerKeyCode::Y => KeyCode::Y,
            SerKeyCode::Z => KeyCode::Z,
            SerKeyCode::LeftBracket => KeyCode::LeftBracket,
            SerKeyCode::Backslash => KeyCode::Backslash,
            SerKeyCode::RightBracket => KeyCode::RightBracket,
            SerKeyCode::GraveAccent => KeyCode::GraveAccent,
            SerKeyCode::World1 => KeyCode::World1,
            SerKeyCode::World2 => KeyCode::World2,
            SerKeyCode::Escape => KeyCode::Escape,
            SerKeyCode::Enter => KeyCode::Enter,
            SerKeyCode::Tab => KeyCode::Tab,
            SerKeyCode::Backspace => KeyCode::Backspace,
            SerKeyCode::Insert => KeyCode::Insert,
            SerKeyCode::Delete => KeyCode::Delete,
            SerKeyCode::Right => KeyCode::Right,
            SerKeyCode::Left => KeyCode::Left,
            SerKeyCode::Down => KeyCode::Down,
            SerKeyCode::Up => KeyCode::Up,
            SerKeyCode::PageUp => KeyCode::PageUp,
            SerKeyCode::PageDown => KeyCode::PageDown,
            SerKeyCode::Home => KeyCode::Home,
            SerKeyCode::End => KeyCode::End,
            SerKeyCode::CapsLock => KeyCode::CapsLock,
            SerKeyCode::ScrollLock => KeyCode::ScrollLock,
            SerKeyCode::NumLock => KeyCode::NumLock,
            SerKeyCode::PrintScreen => KeyCode::PrintScreen,
            SerKeyCode::Pause => KeyCode::Pause,
            SerKeyCode::F1 => KeyCode::F1,
            SerKeyCode::F2 => KeyCode::F2,
            SerKeyCode::F3 => KeyCode::F3,
            SerKeyCode::F4 => KeyCode::F4,
            SerKeyCode::F5 => KeyCode::F5,
            SerKeyCode::F6 => KeyCode::F6,
            SerKeyCode::F7 => KeyCode::F7,
            SerKeyCode::F8 => KeyCode::F8,
            SerKeyCode::F9 => KeyCode::F9,
            SerKeyCode::F10 => KeyCode::F10,
            SerKeyCode::F11 => KeyCode::F11,
            SerKeyCode::F12 => KeyCode::F12,
            SerKeyCode::F13 => KeyCode::F13,
            SerKeyCode::F14 => KeyCode::F14,
            SerKeyCode::F15 => KeyCode::F15,
            SerKeyCode::F16 => KeyCode::F16,
            SerKeyCode::F17 => KeyCode::F17,
            SerKeyCode::F18 => KeyCode::F18,
            SerKeyCode::F19 => KeyCode::F19,
            SerKeyCode::F20 => KeyCode::F20,
            SerKeyCode::F21 => KeyCode::F21,
            SerKeyCode::F22 => KeyCode::F22,
            SerKeyCode::F23 => KeyCode::F23,
            SerKeyCode::F24 => KeyCode::F24,
            SerKeyCode::F25 => KeyCode::F25,
            SerKeyCode::Kp0 => KeyCode::Kp0,
            SerKeyCode::Kp1 => KeyCode::Kp1,
            SerKeyCode::Kp2 => KeyCode::Kp2,
            SerKeyCode::Kp3 => KeyCode::Kp3,
            SerKeyCode::Kp4 => KeyCode::Kp4,
            SerKeyCode::Kp5 => KeyCode::Kp5,
            SerKeyCode::Kp6 => KeyCode::Kp6,
            SerKeyCode::Kp7 => KeyCode::Kp7,
            SerKeyCode::Kp8 => KeyCode::Kp8,
            SerKeyCode::Kp9 => KeyCode::Kp9,
            SerKeyCode::KpDecimal => KeyCode::KpDecimal,
            SerKeyCode::KpDivide => KeyCode::KpDivide,
            SerKeyCode::KpMultiply => KeyCode::KpMultiply,
            SerKeyCode::KpSubtract => KeyCode::KpSubtract,
            SerKeyCode::KpAdd => KeyCode::KpAdd,
            SerKeyCode::KpEnter => KeyCode::KpEnter,
            SerKeyCode::KpEqual => KeyCode::KpEqual,
            SerKeyCode::LeftShift => KeyCode::LeftShift,
            SerKeyCode::LeftControl => KeyCode::LeftControl,
            SerKeyCode::LeftAlt => KeyCode::LeftAlt,
            SerKeyCode::LeftSuper => KeyCode::LeftSuper,
            SerKeyCode::RightShift => KeyCode::RightShift,
            SerKeyCode::RightControl => KeyCode::RightControl,
            SerKeyCode::RightAlt => KeyCode::RightAlt,
            SerKeyCode::RightSuper => KeyCode::RightSuper,
            SerKeyCode::Menu => KeyCode::Menu,
            SerKeyCode::Unknown => KeyCode::Unknown,
        }
    }
}
