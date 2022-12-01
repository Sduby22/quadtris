use macroquad::prelude::*;

use crate::{
    arcade_text::{self, TextDrawer},
    block::{BlockDrawer, BlockVariant},
};

const FONT_PATH: &str = "res/graphics/font_big.png";

pub struct App {
    text_drawer: TextDrawer,
}

impl App {
    pub async fn new() -> Self {
        let text = load_texture(FONT_PATH).await.unwrap();
        let text_drawer = TextDrawer::new(text);
        text.set_filter(FilterMode::Nearest);
        // build_textures_atlas();
        App { text_drawer }
    }

    pub async fn run(&mut self) {
        let img = load_image("res/graphics/blockskin/big/b0.png")
            .await
            .unwrap();
        let drawer = BlockDrawer::new(img);

        loop {
            clear_background(Color::from_rgba(10, 10, 10, 255));

            Self::setup_camera();
            drawer.draw_block(
                BlockVariant::PURPLE,
                Vec3 {
                    x: -50.,
                    y: 0.,
                    z: 0.,
                },
                20.0,
            );

            self.text_drawer
                .draw_text("A", Vec2::ZERO, arcade_text::Color::RED);
            next_frame().await
        }
    }

    fn setup_camera() {
        set_camera(&Camera3D {
            position: Vec3 {
                x: 0.,
                y: 0.,
                z: -300.,
            },
            target: Vec3::ZERO,
            up: -Vec3::Y,
            ..Default::default()
        });
    }
}
