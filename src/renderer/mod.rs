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

impl Renderer {
    pub async fn new() -> Self {
        let block_img = load_image(BLOCK_PATH).await.unwrap();
        let block_renderer = block::BlockRenderer::new(block_img);

        let text_img = load_image(FONT_PATH).await.unwrap();
        let text_renderer = text::TextRenderer::new(text_img);

        Self {
            block_renderer,
            text_renderer,
        }
    }

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
