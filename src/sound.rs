use std::cell::Cell;

use macroquad::audio;

use crate::asset::Assets;

type SoundPtr = Box<dyn Sound>;

pub struct SoundAssets {
    pub mino_spawn: SoundPtr,
    pub mino_lock: SoundPtr,
    pub mino_touch_ground: SoundPtr,
    pub mino_clear: SoundPtr,
    pub mino_hold: SoundPtr,
    pub mino_holdfail: SoundPtr,
}

impl From<&Assets> for SoundAssets {
    fn from(assets: &Assets) -> Self {
        Self {
            mino_spawn: Box::new(RoundRobinSounds::from(vec![
                assets.mino_spawn1,
                assets.mino_spawn2,
                assets.mino_spawn3,
                assets.mino_spawn4,
                assets.mino_spawn5,
                assets.mino_spawn6,
                assets.mino_spawn7,
            ])),
            mino_lock: Box::new(SingleSound::from(assets.mino_lock)),
            mino_touch_ground: Box::new(SingleSound::from(assets.mino_touch_ground)),
            mino_clear: Box::new(SingleSound::from(assets.mino_clear)),
            mino_hold: Box::new(SingleSound::from(assets.mino_hold)),
            mino_holdfail: Box::new(SingleSound::from(assets.mino_holdfail)),
        }
    }
}

pub trait Sound {
    fn play(&self);
}

pub struct SingleSound {
    sound: Option<audio::Sound>,
}

impl From<audio::Sound> for SingleSound {
    fn from(sound: audio::Sound) -> Self {
        Self { sound: Some(sound) }
    }
}

impl Sound for SingleSound {
    fn play(&self) {
        if let Some(s) = self.sound {
            audio::play_sound_once(s);
        }
    }
}

pub struct RoundRobinSounds {
    sounds: Vec<SingleSound>,
    curr: Cell<usize>,
}

impl From<Vec<audio::Sound>> for RoundRobinSounds {
    fn from(sounds: Vec<audio::Sound>) -> Self {
        Self {
            sounds: sounds.into_iter().map(|s| s.into()).collect(),
            curr: Cell::new(0),
        }
    }
}

impl RoundRobinSounds {
    pub fn next(&self) {
        let curr = self.curr.get();
        self.curr.set((curr + 1) % self.sounds.len())
    }

    pub fn play_curr(&self) {
        self.sounds[self.curr.get()].play()
    }
}

impl Sound for RoundRobinSounds {
    fn play(&self) {
        if !self.sounds.is_empty() {
            self.play_curr();
            self.next();
        }
    }
}
