use macroquad::{logging, prelude::*};
use rust_tetris_core::pieces::PieceWithPosition;

use crate::{
    constants::FPS,
    game_data::{GameData, GameState},
    renderer::Renderer,
};

pub struct App {
    renderer: Renderer,
    game_data: GameData,
    time_elapsed: f32,
}

impl App {
    pub async fn new() -> App {
        // build_textures_atlas();
        let renderer = Renderer::new().await;

        App {
            game_data: GameData::new(),
            renderer,
            time_elapsed: 0.,
        }
    }

    pub async fn run(&mut self) {
        self.game_start();
        loop {
            self.time_elapsed += get_frame_time();

            self.tick();
            self.renderer.render(&self.game_data);

            println!("FPS:{}", get_fps());
            next_frame().await;
        }
    }

    fn tick(&mut self) {
        match self.game_data.state {
            GameState::Menu => todo!(),
            GameState::Playing => {
                if is_key_pressed(self.game_data.keybind.restart) {
                    self.game_restart();
                    return;
                }

                if is_key_pressed(self.game_data.keybind.escape) {
                    self.game_stop();
                    return;
                }

                self.game_data.time += get_frame_time();

                if self.game_data.curr_piece.is_some() {
                    self.handle_gravity();
                    self.handle_freeze();
                } else if let Some(p) = self.spawn_piece() {
                    self.game_data.curr_piece = Some(p);
                } else {
                    self.game_data.state = GameState::GameOver;
                }
            }
            GameState::GameOver => todo!(),
        }
    }

    fn handle_gravity(&mut self) {
        let gravity = if is_key_down(self.game_data.keybind.soft_drop) {
            self.game_data.soft_drop_gravity
        } else {
            self.game_data.gravity
        };

        self.game_data.accumulated_down += gravity * relative_frame();
        if self.game_data.accumulated_down >= 1.0 {
            let step = self.game_data.accumulated_down.floor() as usize;
            self.game_data.accumulated_down -= step as f32;
            for _ in 0..step {
                let piece = self.game_data.curr_piece.as_mut().unwrap();
                if piece.collides_down(&self.game_data.board) {
                } else {
                    piece.move_down();
                }
            }
        }
    }

    fn handle_freeze(&mut self) {
        let piece = self.game_data.curr_piece.as_mut().unwrap();
        if piece.collides_down(&self.game_data.board) {
            self.game_data.freeze_left -= relative_frame();
            if self.game_data.freeze_left < 0. {
                self.freeze_piece();
            }
        } else {
            self.game_data.freeze_left = self.game_data.freeze_delay;
        }
    }

    fn spawn_piece(&mut self) -> Option<PieceWithPosition> {
        let p = self.game_data.piece_bag.as_mut().unwrap().next_piece();
        logging::debug!("Spawned piece: {:?}", &p.piece_type);
        let init_pos = match p.piece_type {
            rust_tetris_core::enums::PieceType::I => (3, 18),
            _ => (3, 19),
        };

        let p = PieceWithPosition::new(init_pos.1, init_pos.0, p);
        if p.collides(&self.game_data.board) {
            None
        } else {
            Some(p)
        }
    }

    fn freeze_piece(&mut self) {
        let piece = self.game_data.curr_piece.as_mut().unwrap();
        piece.finalize_on(&mut self.game_data.board);

        self.game_data.curr_piece = None;
        self.game_data.freeze_left = self.game_data.freeze_delay;
    }

    fn game_stop(&mut self) {
        self.game_data.clear();
        self.game_data.state = GameState::Menu;
    }

    fn game_restart(&mut self) {
        self.game_data.clear();
        self.game_data.start();
    }

    fn game_start(&mut self) {
        self.game_data.start();
    }
    //
    // fn draw_ui(&mut self) {
    //     egui_macroquad::ui(|egui_ctx| {
    //         egui_macroquad::egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
    //             ui.style_mut().spacing.slider_width = 500.;
    //             ui.label("Test");
    //             ui.add(Slider::new(&mut self.ui_ctx.fov, 0.0..=1.0).text("slider"));
    //             ui.add(Slider::new(&mut self.ui_ctx.camera_x, 0.0..=1000.0).text("slider"));
    //             ui.add(Slider::new(&mut self.ui_ctx.camera_y, 0.0..=1000.0).text("slider"));
    //             ui.add(Slider::new(&mut self.ui_ctx.camera_z, 0.0..=2000.0).text("slider"));
    //         });
    //     });
    //
    //     egui_macroquad::draw();
    // }
}

fn relative_frame() -> f32 {
    get_frame_time() * 60.
}
