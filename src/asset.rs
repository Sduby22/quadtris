use macroquad::{audio::Sound, prelude::*};
use macroquad_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct Assets {
    #[asset(Image, path = "res/graphics/blockskin/big/b2-sheet.png")]
    pub block_img: Image,

    #[asset(Image, path = "res/graphics/font_big_sheet.png")]
    pub text_img: Image,

    #[asset(Sound, path = "res/se/lock.wav")]
    pub mino_lock: Sound,

    #[asset(Sound, path = "res/se/erase1.wav")]
    pub mino_clear: Sound,

    #[asset(Sound, path = "res/se/hold.wav")]
    pub mino_hold: Sound,

    #[asset(Sound, path = "res/se/holdfail.wav")]
    pub mino_holdfail: Sound,

    #[asset(Sound, path = "res/se/move.wav")]
    pub mino_touch_ground: Sound,

    // 7 mino spawn sounds
    #[asset(Sound, path = "res/se/piece1.wav")]
    pub mino_spawn1: Sound,

    #[asset(Sound, path = "res/se/piece2.wav")]
    pub mino_spawn2: Sound,

    #[asset(Sound, path = "res/se/piece3.wav")]
    pub mino_spawn3: Sound,

    #[asset(Sound, path = "res/se/piece4.wav")]
    pub mino_spawn4: Sound,

    #[asset(Sound, path = "res/se/piece5.wav")]
    pub mino_spawn5: Sound,

    #[asset(Sound, path = "res/se/piece6.wav")]
    pub mino_spawn6: Sound,

    #[asset(Sound, path = "res/se/piece7.wav")]
    pub mino_spawn7: Sound,
}
