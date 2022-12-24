use crate::{constants::FONT_SIZE, game_data::GameData};

use super::{
    text::{self, TextRenderer},
    utils::{pop_model_matrix, push_model_matrix},
};

use macroquad::prelude::*;

pub fn render_score(game_data: &GameData, pos: Vec2, text_renderer: &TextRenderer) {
    push_model_matrix(Mat4::from_translation(Vec3 {
        x: pos.x,
        y: pos.y,
        z: 0.,
    }));

    text_renderer.draw_text("LINES", Vec2::ZERO, FONT_SIZE, text::Color::NAVYBLUE);
    text_renderer.draw_text(
        &game_data.lines.to_string(),
        Vec2::Y * -FONT_SIZE,
        FONT_SIZE,
        text::Color::WHITE,
    );

    text_renderer.draw_text(
        "TIME",
        Vec2::Y * FONT_SIZE * -3.,
        FONT_SIZE,
        text::Color::NAVYBLUE,
    );
    text_renderer.draw_text(
        &time_to_string(game_data.time),
        Vec2::Y * FONT_SIZE * -4.,
        FONT_SIZE,
        text::Color::WHITE,
    );

    pop_model_matrix();
}

// mm:ss:ms
fn time_to_string(time: f32) -> String {
    let minutes = time / 60.;
    let seconds = time as usize % 60;
    let milliseconds = time.fract() * 1000.;

    format!(
        "{:02}:{:02}:{:03}",
        minutes as usize, seconds, milliseconds as usize
    )
}
