use lazy_static::lazy_static;
use macroquad::prelude::*;

/// Camera

pub const CAMERA_FOV: f32 = 0.4;
pub const CAMERA_POS: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 100.,
};

/// UI Size

pub const BLOCK_SIZE: f32 = 1.5;
pub const FONT_SIZE: f32 = 1.5;

pub const BOARD_POS: Vec3 = Vec3 {
    x: -10. * BLOCK_SIZE,
    y: -10. * BLOCK_SIZE,
    z: 0.,
};

pub const SCORE_POS: Vec2 = Vec2 {
    x: 0.,
    y: -FONT_SIZE * 5.,
};

lazy_static! {
    pub static ref MENU_POS: Vec2 = BOARD_POS.xx()
        + Vec2 {
            x: -0.5 * BLOCK_SIZE,
            y: 17.5 * BLOCK_SIZE,
        };
}

/// Default Settings

pub const DAS_DEFAULT: f32 = 8.;
pub const ARR_DEFAULT: f32 = 4.;
pub const SOFT_DROP_DEFAULT: f32 = 0.5;

/// Assets

pub const FIELD_COLS: usize = 10;
pub const FIELD_ROWS: usize = 20;
