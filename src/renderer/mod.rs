use crate::asset::Assets;
use crate::constants::*;
use crate::game_data::{GameData, GameState};
use macroquad::prelude::*;

use self::board::render_board;
use self::score::render_score;

pub mod block;
pub mod board;
pub mod material;
pub mod score;
pub mod text;
pub mod texture_atlas;
pub mod utils;

pub struct Renderer {
    pub block_renderer: block::BlockRenderer,
    pub text_renderer: text::TextRenderer,
}

impl From<&Assets> for Renderer {
    fn from(assets: &Assets) -> Self {
        let block_renderer = block::BlockRenderer::new(assets.block_img.clone());
        let text_renderer = text::TextRenderer::new(assets.text_img.clone());

        Self {
            block_renderer,
            text_renderer,
        }
    }
}

impl Renderer {
    pub fn render(&self, game_data: &GameData) {
        clear_background(Color::from_rgba(10, 10, 10, 255));
        self.setup_camera();

        render_board(
            game_data,
            BOARD_POS,
            &self.block_renderer,
            &self.text_renderer,
        );

        if game_data.state != GameState::Menu {
            render_score(game_data, SCORE_POS, &self.text_renderer);
        }
    }

    fn setup_camera(&self) {
        set_camera(&Camera3D {
            position: CAMERA_POS,
            fovy: CAMERA_FOV,
            target: Vec3::ZERO,
            up: Vec3::Y,
            ..Default::default()
        });
    }
}
