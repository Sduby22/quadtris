use macroquad::prelude::*;

pub const BLOCK_SIZE: f32 = 3.0;
pub const FONT_SIZE: f32 = 3.0;
pub const FONT_PATH: &str = "res/graphics/font_big_sheet.png";
pub const BLOCK_PATH: &str = "res/graphics/blockskin/big/b2-sheet.png";

pub const BOARD_POS: Vec3 = Vec3 {
    x: -30.,
    y: -30.,
    z: 0.,
};

pub const SCORE_POS: Vec2 = Vec2 {
    x: 0.,
    y: -FONT_SIZE * 5.,
};

pub const DAS_DEFAULT: f32 = 8.;
pub const ARR_DEFAULT: f32 = 4.;
pub const SOFT_DROP_DEFAULT: f32 = 0.5;
