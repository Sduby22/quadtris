use macroquad::prelude::*;

use super::texture_atlas::TextureAltas;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Color {
    White,
    Blue,
    Red,
    Magenta,
    Green,
    Cream,
    Cyan,
    Yellow,
    Purple,
    Navyblue,
}

pub struct TextRenderer {
    texture_atlas: TextureAltas,
}

impl TextRenderer {
    pub fn new(image: Image) -> Self {
        TextRenderer {
            texture_atlas: TextureAltas::new(image, 28, 29, 32, 32, 2, 2),
        }
    }

    pub fn draw_text(&self, text: &str, position: Vec2, size: f32, color: Color) {
        let mut x = position.x;
        let y = position.y;
        for c in text.chars() {
            let c = c.to_uppercase().next().unwrap();
            let code = c as usize;

            // continue if c is not in valid range
            if !(32..=96).contains(&code) {
                warn!("Character {} is not in valid range", c);
                continue;
            }

            // ASCII 32 is space
            if code != b' ' as usize {
                let glyph_code = code - 33;
                self.draw_character(glyph_code, Vec2 { x, y }, size, color);
            }
            x += size;
        }
    }

    fn draw_character(&self, variant: usize, position: Vec2, size: f32, color: Color) {
        let index = variant + color as usize * 80;

        draw_texture_ex(
            self.texture_atlas.get_texture(index),
            position.x,
            position.y,
            macroquad::color::Color::from_rgba(255, 255, 255, 255),
            DrawTextureParams {
                dest_size: Some(Vec2 { x: size, y: size }),
                flip_x: false,
                flip_y: true,
                ..Default::default()
            },
        )
    }
}
