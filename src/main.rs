use macroquad::prelude::*;

use arcade_text::TextDrawer;
mod app;
mod arcade_text;

#[macroquad::main("Tetris")]
async fn main() {
    let mut app = app::App::new().await;

    app.run().await;
}
