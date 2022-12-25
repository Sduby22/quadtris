use macroquad::prelude::*;

pub const BLOCK_SIZE: f32 = 3.0;
pub const FONT_SIZE: f32 = 3.0;
pub const FONT_PATH: &str = "res/graphics/font_big.png";
pub const BLOCK_PATH: &str = "res/graphics/blockskin/big/b2.png";

pub const BOARD_POS: Vec3 = Vec3 {
    x: -30.,
    y: -30.,
    z: 0.,
};

pub const SCORE_POS: Vec2 = Vec2 {
    x: 0.,
    y: -FONT_SIZE * 5.,
};
