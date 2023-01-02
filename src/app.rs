use lazy_static::lazy_static;
use macroquad::{logging, prelude::*};
use rust_tetris_core::{
    board::Board,
    holder::{HoldPiece, Swappable},
    pieces::{Piece, PieceWithPosition},
};

use crate::{
    constants::MENU_POS,
    game_data::{load_user_settings, save_user_settings, GameData, GameState, MoveState},
    menu::*,
    renderer::{text, Renderer},
    sound::SoundAssets,
};

pub struct App {
    renderer: Renderer,
    game_data: GameData,
    sounds: SoundAssets,
    menu_ctx: MenuCtx,
    time_elapsed: f32,
}

impl App {
    pub async fn new() -> App {
        // build_textures_atlas();
        let renderer = Renderer::new().await;

        let sounds = SoundAssets::new().await;
        let mut game_data = GameData::new();

        load_user_settings(&mut game_data);

        App {
            game_data,
            renderer,
            sounds,
            menu_ctx: MenuCtx::new(),
            time_elapsed: 0.,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.time_elapsed += get_frame_time();

            self.tick();
            self.renderer.render(&self.game_data);

            match self.game_data.state {
                GameState::Menu => {
                    self.draw_menu();
                }
                GameState::GameOver => {
                    self.draw_gameover();
                }
                _ => (),
            }

            next_frame().await;
        }
    }

    fn tick(&mut self) {
        match self.game_data.state {
            GameState::Menu => {}
            GameState::Playing => {
                if self.game_data.keybind.restart.is_pressed() {
                    self.game_restart();
                    return;
                }

                if self.game_data.keybind.escape.is_pressed() {
                    self.game_stop();
                    return;
                }

                self.game_data.time += get_frame_time();

                if self.game_data.curr_piece.is_none() {
                    self.game_data.spawn_delay_timer.tick(relative_frame());
                    if self.game_data.spawn_delay_timer.done() {
                        if let Some(p) = self.spawn_piece() {
                            self.game_data.curr_piece = Some(p);
                        } else {
                            self.game_over();
                        }
                        self.game_data.spawn_delay_timer.reset();
                    } else {
                        self.handle_irs();
                    }
                }

                self.handle_move();
                self.handle_hold();
                self.handle_rotate();
                self.handle_gravity();
                self.handle_freeze();
            }
            GameState::GameOver => {
                if self.game_data.keybind.restart.is_pressed() {
                    self.game_restart();
                    return;
                }

                if self.game_data.keybind.escape.is_pressed() {
                    self.game_stop();
                    return;
                }
            }
        }
    }

    pub fn draw_gameover(&self) {
        let ctx = &mut MenuCtx::new();
        let mut menu = Menu::new(ctx, &self.renderer.text_renderer);

        menu.add_widget(Label::new("GAME OVER!", text::Color::Yellow));
        menu.draw(*MENU_POS);
    }

    pub fn draw_menu(&mut self) {
        let ctx = &mut self.menu_ctx;

        let mut next_state = ctx.curr_state();
        let mut menu = Menu::new(ctx, &self.renderer.text_renderer);
        match next_state {
            MenuState::Main => {
                menu.add_widget(Button::new("START", || {
                    self.game_data.start();
                }));
                menu.add_widget(Button::new("SETTINGS", || {
                    next_state = MenuState::Settings;
                }));
            }
            MenuState::Settings => {
                menu.add_widget(KeyBind::new(&mut self.game_data.keybind.left, "LEFT"));
                menu.add_widget(KeyBind::new(&mut self.game_data.keybind.right, "RIGHT"));
                menu.add_widget(KeyBind::new(
                    &mut self.game_data.keybind.soft_drop,
                    "SOFT DROP",
                ));
                menu.add_widget(KeyBind::new(
                    &mut self.game_data.keybind.hard_drop,
                    "HARD DROP",
                ));
                menu.add_widget(KeyBind::new(
                    &mut self.game_data.keybind.rotate_cw,
                    "ROTATE CW",
                ));
                menu.add_widget(KeyBind::new(
                    &mut self.game_data.keybind.rotate_ccw,
                    "ROTATE CCW",
                ));
                menu.add_widget(KeyBind::new(
                    &mut self.game_data.keybind.rotate_180,
                    "ROTATE 180",
                ));
                menu.add_widget(KeyBind::new(&mut self.game_data.keybind.hold, "HOLD"));
                menu.add_widget(KeyBind::new(&mut self.game_data.keybind.restart, "RESTART"));

                menu.add_widget(Margin);

                menu.add_widget(Selector::new(
                    "DAS",
                    self.game_data.das_timer.get_duration_mut(),
                    &DAS_VALUES,
                    &DAS_LABELS,
                ));
                menu.add_widget(Selector::new(
                    "ARR",
                    &mut self.game_data.arr,
                    &ARR_VALUES,
                    &ARR_LABELS,
                ));
                menu.add_widget(Selector::new(
                    "SOFTDROP",
                    &mut self.game_data.soft_drop_gravity,
                    &SOFT_DROP_VALUES,
                    &SOFT_DROP_LABELS,
                ));
            }
        }

        menu.draw(*MENU_POS);
        let menu_len = menu.len();
        drop(menu);

        ctx.push_state(next_state);

        if get_last_key_pressed().is_some() && ctx.modifying {
            let k = get_last_key_pressed().unwrap();
            if k != KeyCode::Enter {
                ctx.modifying = false;
            }
        } else if is_key_pressed(KeyCode::Down) {
            ctx.curr_pointer += 1;
        } else if is_key_pressed(KeyCode::Up) {
            ctx.curr_pointer -= 1;
        } else if is_key_pressed(KeyCode::Escape) {
            if let MenuState::Settings = ctx.curr_state() {
                save_user_settings(&self.game_data);
            }
            ctx.pop_state();
        }

        ctx.curr_pointer = ctx.curr_pointer.clamp(0, menu_len as i32 - 1);
    }

    fn handle_gravity(&mut self) {
        let Some(piece) = &mut self.game_data.curr_piece else {return};
        let gravity = if self.game_data.keybind.soft_drop.is_down() {
            self.game_data.soft_drop_gravity.max(self.game_data.gravity)
        } else {
            self.game_data.gravity
        };

        let touch_ground_before = piece.collides_down(&self.game_data.board);
        self.game_data.accumulated_down += gravity * relative_frame();
        if self.game_data.accumulated_down >= 1.0 {
            let step = self.game_data.accumulated_down.floor() as usize;
            self.game_data.accumulated_down -= step as f32;
            for _ in 0..step {
                if piece.collides_down(&self.game_data.board) {
                    break;
                } else {
                    piece.move_down();
                }
            }
        }

        if !touch_ground_before && piece.collides_down(&self.game_data.board) {
            self.sounds.mino_touch_ground.play();
        }
    }

    fn handle_freeze(&mut self) {
        let Some(piece) = &mut self.game_data.curr_piece else {return};
        let freeze_timer = &mut self.game_data.freeze_timer;
        if piece.collides_down(&self.game_data.board) {
            freeze_timer.tick(relative_frame());
            if freeze_timer.done() {
                self.freeze_piece();
            }
        } else {
            freeze_timer.reset();
        }
    }

    fn handle_move(&mut self) {
        if self.game_data.keybind.hard_drop.is_pressed() {
            self.hard_drop();
            return;
        }

        let first_move = if self.game_data.keybind.left.is_pressed() {
            self.change_move_state(MoveState::Left);
            true
        } else if self.game_data.keybind.right.is_pressed() {
            self.change_move_state(MoveState::Right);
            true
        } else {
            false
        };

        if let Some(piece) = &mut self.game_data.curr_piece {
            if first_move {
                piece_move_step(
                    piece,
                    self.game_data.move_state,
                    1,
                    &self.game_data.board,
                    &self.sounds,
                );
            }
        }

        match self.game_data.move_state {
            MoveState::Left if self.game_data.keybind.left.is_down() => self.handle_das(),
            MoveState::Right if self.game_data.keybind.right.is_down() => self.handle_das(),
            _ => {
                self.change_move_state(MoveState::No);
            }
        }
    }

    fn handle_hold(&mut self) {
        if self.game_data.keybind.hold.is_pressed() {
            if self.game_data.hold_piece.can_swap() {
                // hold success
                let Some(piece) = self.game_data.curr_piece.take() else { return };

                self.game_data.curr_piece = if let Some(p) = self.game_data.hold_piece.take() {
                    self.init_piece(p.piece)
                } else {
                    self.spawn_piece()
                };

                let mut hp = HoldPiece::new(piece.tetris_piece());
                hp.set_hold();
                self.game_data.hold_piece = Some(hp);
                self.sounds.mino_hold.play();
            } else {
                // hold failed
                self.sounds.mino_holdfail.play();
            }
        }
    }

    fn handle_rotate(&mut self) {
        let Some(piece) = &mut self.game_data.curr_piece else {return};

        let kb = &self.game_data.keybind;
        let touch_ground_before = piece.collides_down(&self.game_data.board);

        let rotated = if kb.rotate_cw.is_pressed() {
            piece.try_rotate(&self.game_data.board)
        } else if kb.rotate_ccw.is_pressed() {
            piece.try_rotate_prev(&self.game_data.board)
        } else if kb.rotate_180.is_pressed() {
            piece.try_rotate_180(&self.game_data.board)
        } else {
            false
        };

        if rotated && touch_ground_before {
            self.sounds.mino_touch_ground.play()
        }
    }

    /// handle rotation input during spawn delay
    fn handle_irs(&mut self) {
        let next_piece = self
            .game_data
            .piece_bag
            .as_mut()
            .unwrap()
            .next_piece_ref_mut();
        let kb = &self.game_data.keybind;
        if kb.rotate_cw.is_pressed() {
            next_piece.rotate_piece()
        } else if kb.rotate_ccw.is_pressed() {
            next_piece.rotate_piece_prev()
        } else if kb.rotate_180.is_pressed() {
            next_piece.rotate_piece_180()
        };
    }

    fn hard_drop(&mut self) {
        let Some(piece) = &mut self.game_data.curr_piece else {return};

        while !piece.collides_down(&self.game_data.board) {
            piece.move_down();
        }

        self.freeze_piece();
        self.game_data.curr_piece = None;

        self.sounds.mino_touch_ground.play();
    }

    fn change_move_state(&mut self, state: MoveState) {
        self.game_data.move_state = state;
        self.game_data.das_timer.reset();
        self.game_data.accumulated_move = 0.;
    }

    fn handle_das(&mut self) {
        if self.game_data.das_timer.done() {
            self.game_data.accumulated_move += relative_frame() / self.game_data.arr.max(0.000001);
            if let Some(piece) = &mut self.game_data.curr_piece {
                let step = self.game_data.accumulated_move.floor() as usize;
                self.game_data.accumulated_move = self.game_data.accumulated_move.fract();
                piece_move_step(
                    piece,
                    self.game_data.move_state,
                    step,
                    &self.game_data.board,
                    &self.sounds,
                );
            }
        } else {
            self.game_data.das_timer.tick(relative_frame());
        }
    }

    fn handle_clear(&mut self) {
        let board = &mut self.game_data.board;
        let ranges = board.completed_rows();
        if !ranges.is_empty() {
            for (from, to) in ranges.iter() {
                self.game_data.lines += (from - to) as u32;
            }

            board.remove_ranges(ranges);
            self.sounds.mino_clear.play();
        }
    }

    fn spawn_piece(&mut self) -> Option<PieceWithPosition> {
        if let Some(hp) = &mut self.game_data.hold_piece {
            hp.reset_hold();
        }
        let p = self.game_data.piece_bag.as_mut().unwrap().next_piece();
        logging::debug!("Spawned piece: {:?}", &p.piece_type);

        self.sounds.mino_spawn.play();
        self.init_piece(p)
    }

    fn init_piece(&mut self, p: Piece) -> Option<PieceWithPosition> {
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
        self.game_data.freeze_timer.reset();

        self.sounds.mino_lock.play();
        self.handle_clear();
    }

    fn game_stop(&mut self) {
        self.game_data.clear();
        self.game_data.state = GameState::Menu;
    }

    fn game_over(&mut self) {
        self.game_data.state = GameState::GameOver;
    }

    fn game_restart(&mut self) {
        self.game_data.clear();
        self.game_data.start();
    }

    fn game_start(&mut self) {
        self.game_data.start();
    }
}

fn relative_frame() -> f32 {
    get_frame_time() * 60.
}

fn piece_move_step(
    piece: &mut PieceWithPosition,
    move_state: MoveState,
    mut step: usize,
    board: &Board,
    sounds: &SoundAssets,
) {
    let mut moved = false;
    let touch_ground_before = piece.collides_down(board);

    match move_state {
        MoveState::Left => {
            while step != 0 && !piece.collides_left(board) {
                step -= 1;
                piece.move_left();
                moved = true;
            }
        }
        MoveState::Right => {
            while step != 0 && !piece.collides_right(board) {
                step -= 1;
                piece.move_right();
                moved = true;
            }
        }
        _ => {}
    }

    let touch_groud_after = piece.collides_down(board);

    if moved && touch_ground_before && touch_groud_after {
        sounds.mino_touch_ground.play();
    }
}

lazy_static! {
    static ref DAS_VALUES: Vec<f32> = (1..=15).map(|x| x as f32).collect();
    static ref DAS_LABELS: Vec<String> = DAS_VALUES.iter().map(|x| x.to_string()).collect();
    static ref ARR_VALUES: Vec<f32> = (0..=10).map(|x| x as f32).collect();
    static ref ARR_LABELS: Vec<String> = ARR_VALUES.iter().map(|x| x.to_string()).collect();
    static ref SOFT_DROP_VALUES: Vec<f32> = vec![
        1. / 64.,
        1. / 32.,
        1. / 16.,
        1. / 8.,
        1. / 4.,
        1. / 2.,
        1.,
        2.,
        3.,
        4.,
        5.,
        20.
    ];
    static ref SOFT_DROP_LABELS: Vec<String> = vec![
        "1/64G".to_string(),
        "1/32G".to_string(),
        "1/16G".to_string(),
        "1/8G".to_string(),
        "1/4G".to_string(),
        "1/2G".to_string(),
        "1G".to_string(),
        "2G".to_string(),
        "3G".to_string(),
        "4G".to_string(),
        "5G".to_string(),
        "20G".to_string(),
    ];
}
