use rodio::source::Source;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use synth::Synth;

pub mod env;
pub mod lfo;
pub mod osc;
pub mod synth;

pub const SAMPLE_RATE: u32 = 48_000;

pub struct Player {
    pub synth: Arc<Mutex<Synth>>,
}

impl Iterator for Player {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.synth.lock().unwrap().get_sample())
    }
}

impl Source for Player {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
