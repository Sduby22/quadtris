use macroquad::{miniquad::CullFace, prelude::*};

#[derive(Clone, Copy)]
pub enum BlockVariant {
    GRAY,
    WHITE,

    // 7 tetrimino variants
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    CYAN,
    BLUE,
    PURPLE,
}

pub struct BlockRenderer {
    textures: Vec<Texture2D>,
}

impl BlockRenderer {
    pub fn new(block_img: Image) -> Self {
        let mut textures = vec![];
        for i in 0..9 {
            let img = block_img.sub_image(Rect::new((32 * i) as f32, 0.0, 32.0, 32.0));
            let t = Texture2D::from_image(&img);
            t.set_filter(FilterMode::Nearest);
            textures.push(t);
        }
        Self { textures }
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
        self.textures[variant as usize]
    }
}
