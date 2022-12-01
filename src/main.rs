use macroquad::prelude::*;

use arcade_text::TextDrawer;
mod app;
mod arcade_text;
mod block;

#[macroquad::main("Tetris")]
async fn main() {
    let mut app = app::App::new().await;

    app.run().await;
}
