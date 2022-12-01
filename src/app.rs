use macroquad::prelude::*;

use crate::arcade_text::{self, TextDrawer};

pub struct App {
    text_drawer: TextDrawer,
}

impl App {
    pub async fn new() -> Self {
        let text = load_texture("asset/font.png").await.unwrap();
        let text_drawer = TextDrawer::new(text);
        text.set_filter(FilterMode::Nearest);
        // build_textures_atlas();
        App { text_drawer }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(Color::from_rgba(10, 10, 10, 255));

            Self::setup_camera();

            draw_cube(
                Vec3 {
                    x: 20.,
                    y: 0.,
                    z: 0.,
                },
                Vec3 {
                    x: 10.,
                    y: 10.0,
                    z: 10.0,
                },
                None,
                Color::from_rgba(255, 0, 0, 255),
            );

            self.text_drawer
                .draw_text("ASDaE", Vec2::ZERO, arcade_text::Color::MAGENTA);
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
            projection: Projection::Perspective,
            ..Default::default()
        });
    }
}
