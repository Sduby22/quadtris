mod app;
mod constants;
mod game_data;
mod menu;
mod renderer;
mod sound;
mod asset;

#[macroquad::main("Tetris")]
async fn main() {
    let mut app = app::App::new().await;

    app.run().await;
}
