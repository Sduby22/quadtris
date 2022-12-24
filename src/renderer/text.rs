use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Color {
    WHITE,
    BLUE,
    RED,
    MAGENTA,
    GREEN,
    CREAM,
    CYAN,
    YELLOW,
    PURPLE,
    NAVYBLUE,
}

pub struct TextRenderer {
    text: Texture2D,
}

impl TextRenderer {
    pub fn new(text: Texture2D) -> Self {
        TextRenderer { text }
    }

    pub fn draw_text(&self, text: &str, position: Vec2, size: f32, color: Color) {
        let mut x = position.x;
        let y = position.y;
        for c in text.chars() {
            let code = c as usize;

            // continue if c is not in valid range
            if !(32..=96).contains(&code) {
                warn!("Character {} is not in valid range", c);
                continue;
            }

            let glyph_code = code - 32;
            self.draw_character(glyph_code, Vec2 { x, y }, size, color);
            x += size;
        }
    }

    fn draw_character(&self, variant: usize, position: Vec2, size: f32, color: Color) {
        let row = color as usize * 3 + variant / 32;
        let col = variant % 32;

        let x = col as f32 * 32.0;
        let y = row as f32 * 32.0;

        draw_texture_ex(
            self.text,
            position.x,
            position.y,
            macroquad::color::Color::from_rgba(255, 255, 255, 255),
            DrawTextureParams {
                source: Some(Rect {
                    x,
                    y,
                    w: 32.,
                    h: 32.,
                }),
                dest_size: Some(Vec2 { x: size, y: size }),
                flip_x: false,
                flip_y: true,
                ..Default::default()
            },
        )
    }
}
