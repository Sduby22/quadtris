use macroquad::prelude::*;

use super::texture_atlas::TextureAltas;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum BlockVariant {
    Gray,
    White,

    // 7 tetrimino variants
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
}

pub struct BlockRenderer {
    texture_atlas: TextureAltas,
}

impl BlockRenderer {
    pub fn new(block_img: Image) -> Self {
        Self {
            texture_atlas: TextureAltas::new(block_img, 1, 9, 32, 32, 2, 2),
        }
    }

    pub fn draw_block(&self, variant: BlockVariant, position: Vec3, size: f32) {
        let text = self.get_texture(variant);
        draw_cube(
            position,
            Vec3::new(size, size, size),
            text,
            Color::from_rgba(255, 255, 255, 255),
        );
    }

    pub fn draw_wire_block(&self, position: Vec3, size: f32) {
        draw_cube_wires(
            position,
            Vec3::new(size, size, size),
            Color::from_rgba(255, 255, 255, 255),
        );
    }

    fn get_texture(&self, variant: BlockVariant) -> Texture2D {
        self.texture_atlas.get_texture(variant as usize)
    }
}
