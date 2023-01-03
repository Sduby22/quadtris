use std::cell::Cell;

use macroquad::{audio, logging, prelude::*};

use crate::constants::*;

type SoundPtr = Box<dyn Sound>;

pub struct SoundAssets {
    pub mino_spawn: SoundPtr,
    pub mino_lock: SoundPtr,
    pub mino_touch_ground: SoundPtr,
    pub mino_clear: SoundPtr,
    pub mino_hold: SoundPtr,
    pub mino_holdfail: SoundPtr,
}

impl SoundAssets {
    pub async fn new() -> Self {
        Self {
            mino_spawn: Box::new(RoundRobinSounds::new(MINO_SPAWN.clone()).await),
            mino_lock: Box::new(SingleSound::new(MINO_LOCK.into()).await),
            mino_touch_ground: Box::new(SingleSound::new(MINO_TOUCH_GROUND.into()).await),
            mino_clear: Box::new(SingleSound::new(MINO_CLEAR.into()).await),
            mino_hold: Box::new(SingleSound::new(MINO_HOLD.into()).await),
            mino_holdfail: Box::new(SingleSound::new(MINO_HOLDFAIL.into()).await),
        }
    }
}

pub trait Sound {
    fn play(&self);
}

pub struct SingleSound {
    sound: Option<audio::Sound>,
}

impl SingleSound {
    pub async fn new(path: String) -> Self {
        let sound = audio::load_sound(&path).await.ok();
        if sound.is_none() {
            logging::warn!("Can't load {}", path);
        }
        Self { sound }
    }

    pub fn is_some(&self) -> bool {
        self.sound.is_some()
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

impl RoundRobinSounds {
    pub async fn new(paths: Vec<String>) -> Self {
        let mut sounds = vec![];

        let coroutine_vec: Vec<_> = paths
            .into_iter()
            .map(|p| coroutines::start_coroutine(SingleSound::new(p)))
            .collect();

        for c in coroutine_vec {
            while !c.is_done() {
                next_frame().await;
            }
            if let Some(s) = c.retrieve() {
                sounds.push(s);
            }
        }

        Self {
            sounds,
            curr: Cell::new(0),
        }
    }

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
