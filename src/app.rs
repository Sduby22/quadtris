use egui_macroquad::egui::Slider;
use macroquad::prelude::*;
use rust_tetris_core::{board::Board, piece_bag::PieceBag, test_utils::load_board};

use crate::{game_data::GameData, renderer::Renderer};

struct UiCtx {
    fov: f32,
    camera_x: f32,
    camera_y: f32,
    camera_z: f32,
}

pub struct App {
    renderer: Renderer,
    game_data: GameData,
    ui_ctx: UiCtx,
}

impl App {
    pub async fn new() -> App {
        // build_textures_atlas();
        let mut board = Board::new(20, 10);
        load_board(&mut board, "************");
        let game_data = GameData {
            // board: Board::new(20, 10),
            board,
            piece_bag: PieceBag::new(6, None),
        };
        let renderer = Renderer::new().await;

        App {
            game_data,
            renderer,
            ui_ctx: UiCtx {
                fov: 0.3,
                camera_x: 0.0,
                camera_y: 0.0,
                camera_z: 300.0,
            },
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.render();

            next_frame().await
        }
    }

    fn draw_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui_macroquad::egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.style_mut().spacing.slider_width = 500.;
                ui.label("Test");
                ui.add(Slider::new(&mut self.ui_ctx.fov, 0.0..=1.0).text("slider"));
                ui.add(Slider::new(&mut self.ui_ctx.camera_x, 0.0..=1000.0).text("slider"));
                ui.add(Slider::new(&mut self.ui_ctx.camera_y, 0.0..=1000.0).text("slider"));
                ui.add(Slider::new(&mut self.ui_ctx.camera_z, 0.0..=2000.0).text("slider"));
            });
        });

        egui_macroquad::draw();
    }

    fn render(&mut self) {
        clear_background(Color::from_rgba(10, 10, 10, 255));

        self.setup_camera();
        self.renderer.render(&self.game_data);
        self.draw_ui();
    }

    fn setup_camera(&self) {
        set_camera(&Camera3D {
            position: Vec3 {
                x: self.ui_ctx.camera_x,
                y: self.ui_ctx.camera_y,
                z: self.ui_ctx.camera_z,
            },
            fovy: self.ui_ctx.fov,
            target: Vec3::ZERO,
            up: Vec3::Y,
            ..Default::default()
        });
    }
}
