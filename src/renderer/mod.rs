use crate::constants::*;
use crate::game_data::GameData;
use macroquad::prelude::*;

use self::board::render_board;

mod block;
mod board;
mod text;
mod utils;

pub struct Renderer {
    block_renderer: block::BlockRenderer,
    text_renderer: text::TextRenderer,
}

impl Renderer {
    pub async fn new() -> Self {
        let block_img = load_image(BLOCK_PATH).await.unwrap();
        let block_renderer = block::BlockRenderer::new(block_img);

        let text_img = load_texture(FONT_PATH).await.unwrap();
        text_img.set_filter(FilterMode::Nearest);
        let text_renderer = text::TextRenderer::new(text_img);

        Self {
            block_renderer,
            text_renderer,
        }
    }

    pub fn render(&self, game_data: &GameData) {
        draw_grid(
            5,
            5.0,
            Color::from_rgba(255, 255, 255, 255),
            Color::from_rgba(255, 0, 0, 255),
        );
        render_board(
            game_data,
            Vec3 {
                x: -30.,
                y: -30.,
                z: 0.,
            },
            &self.block_renderer,
            &self.text_renderer,
        );
        self.draw_text("A", Vec2::Y * 10., FONT_SIZE, text::Color::RED);
    }

    pub fn draw_block(&self, variant: block::BlockVariant, position: Vec3, size: f32) {
        self.block_renderer.draw_block(variant, position, size);
    }

    pub fn draw_text(&self, text: &str, position: Vec2, size: f32, color: text::Color) {
        self.text_renderer.draw_text(text, position, size, color);
    }
}
