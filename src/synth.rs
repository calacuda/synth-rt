use crate::{
    chorus::Chorus,
    lfo::LFO,
    osc::{Oscillator, Overtone},
};
use midi_control::MidiNote;
use std::{ops::Index, sync::Arc};

pub type WaveTable = Arc<[f32]>;
// pub type WaveTables = [(WaveTable, f32); 2];

pub const WAVE_TABLE_SIZE: usize = 256;
pub const VOICES: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OscType {
    Sin,
    Tri,
    Sqr,
    Saw,
}

pub struct WaveTables {
    pub sin: WaveTable,
    pub tri: WaveTable,
    pub sqr: WaveTable,
    pub saw: WaveTable,
}

impl WaveTables {
    pub fn new(overtones: &[Overtone]) -> Self {
        Self {
            sin: Self::build_sine_table(overtones),
            tri: Self::build_triangle_table(overtones),
            sqr: Self::build_square_table(overtones),
            saw: Self::build_saw_table(overtones),
        }
    }

    fn build_saw_table(overtones: &[Overtone]) -> WaveTable {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let n_overtones = overtones.len();

        let bias = 1.0 / n_overtones as f32;

        for i in 0..WAVE_TABLE_SIZE {
            for ot in overtones {
                // wave_table[i] += (((i as f64 % ot.overtone) - 1.0) * ot.volume) as f32
                wave_table[i] +=
                    ((((i as f64 * ((4.0 * ot.overtone) / WAVE_TABLE_SIZE as f64)) % 2.0) - 1.0)
                        * ot.volume) as f32;
                // break;
            }

            wave_table[i] *= bias;
            // println!("saw tooth => {}", wave_table[i]);
        }

        wave_table.into()
    }

    fn build_square_table(overtones: &[Overtone]) -> WaveTable {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let n_overtones = overtones.len();

        let bias = 1.0 / n_overtones as f32;

        for i in 0..WAVE_TABLE_SIZE {
            for ot in overtones {
                if (i as f64 % ot.overtone as f64) < 1.0 {
                    wave_table[i] += ot.volume as f32
                }
            }

            wave_table[i] *= bias;
        }

        wave_table.into()
    }

    fn build_triangle_table(overtones: &[Overtone]) -> WaveTable {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let n_overtones = overtones.len();

        let bias = 1.0 / n_overtones as f32;

        for i in 0..WAVE_TABLE_SIZE {
            for ot in overtones {
                wave_table[i] += (((i as f64 % ot.overtone as f64) - 1.0).abs() * ot.volume) as f32
            }

            wave_table[i] *= bias;
        }

        // println!("bigest build_triangle_table {:?}", wave_table.iter().max());

        wave_table.into()
    }

    fn build_sine_table(overtones: &[Overtone]) -> WaveTable {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let n_overtones = overtones.len();

        let bias = 1.0 / n_overtones as f32;

        for i in 0..WAVE_TABLE_SIZE {
            for ot in overtones {
                wave_table[i] += ((2.0 * core::f64::consts::PI * i as f64 * ot.overtone
                    / WAVE_TABLE_SIZE as f64)
                    .sin()
                    * ot.volume) as f32
            }

            wave_table[i] *= bias;
        }

        wave_table.into()
    }

    fn index(&self, index: &Arc<[(OscType, f32)]>) -> Arc<[(WaveTable, f32)]> {
        index
            .iter()
            .map(|(osc_type, vol)| {
                (
                    match osc_type {
                        OscType::Sin => self.sin.clone(),
                        OscType::Tri => self.tri.clone(),
                        OscType::Sqr => self.sqr.clone(),
                        OscType::Saw => self.saw.clone(),
                    },
                    vol / index.len() as f32,
                )
            })
            .collect()
    }
}

pub struct Synth {
    osc_s: [Oscillator; VOICES],
    wave_tables: WaveTables,
    osc_type: Arc<[(OscType, f32)]>,
    lfo: LFO,
    volume: f32,
    chorus: Chorus,
}

impl Synth {
    pub fn new() -> Self {
        let overtones = [
            Overtone {
                overtone: 1.0_f64.powf(1.0 / 12.0),
                volume: 1.0,
            },
            Overtone {
                // overtone: 2.0_f64.powf(1.0 / 12.0),
                overtone: 2.5_f64.powf(1.0 / 12.0),
                volume: 1.0,
            },
            Overtone {
                overtone: 2.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 3.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 4.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 5.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 6.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 8.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 9.0,
                volume: 1.0,
            },
            Overtone {
                overtone: 10.0,
                volume: 1.0,
            },
        ];
        let wave_tables = WaveTables::new(&overtones);
        let mut lfo = LFO::new();
        lfo.set_frequency(400.0 / 60.0);

        Self {
            osc_s: [Oscillator::new(); VOICES],
            wave_tables,
            osc_type: Arc::new([
                // (OscType::Sin, 1.0),
                (OscType::Saw, 1.0),
                // (OscType::Tri, 0.75),
                // (OscType::Sqr, 1.0),
            ]),
            // osc_type: Arc::new([(OscType::Tri, 1.0)]),
            lfo,
            volume: 0.75,
            chorus: Chorus::new(),
        }
    }

    pub fn set_overtones(&mut self, overtones: &[Overtone]) {
        self.wave_tables = WaveTables::new(overtones);
    }

    pub fn get_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        let lfo_sample = self.lfo.get_sample();
        // println!("lfo sample {lfo_sample}");

        for osc in self.osc_s.iter_mut() {
            // println!("{osc:?}");
            if osc.playing.is_some() {
                osc.vibrato(lfo_sample);
                // println!("playing");
                sample += osc.get_sample(&self.wave_tables.index(&self.osc_type));
                // println!(
                //     "env => {}, {}",
                //     osc.env_filter.get_samnple(),
                //     osc.env_filter.phase
                // );
            }
        }

        let sample = sample * (self.volume + lfo_sample * 0.0125);
        sample + self.chorus.get_sample(sample)
        // println!("synth sample => {sample}");
        // sample * self.volume
    }

    pub fn play(&mut self, midi_note: MidiNote, velocity: u8) {
        let midi_note = if midi_note >= 12 {
            midi_note - 12
        } else {
            return;
        };

        for osc in self.osc_s.iter_mut() {
            if osc.playing == Some(midi_note) {
                return;
            }
        }

        for osc in self.osc_s.iter_mut() {
            if osc.playing.is_none() {
                osc.press(midi_note);
                // println!("playing note on osc {i}");

                break;
            }
        }
    }

    pub fn stop(&mut self, midi_note: MidiNote) {
        let midi_note = if midi_note >= 12 {
            midi_note - 12
        } else {
            return;
        };

        for osc in self.osc_s.iter_mut() {
            if osc.playing == Some(midi_note) {
                // println!("release");
                osc.release();
                break;
            }
        }
    }

    pub fn bend_all(&mut self, bend: f32) {
        for osc in self.osc_s.iter_mut() {
            if osc.playing.is_some() {
                osc.bend(bend);
            }
        }
    }

    pub fn unbend(&mut self) {
        for osc in self.osc_s.iter_mut() {
            if osc.playing.is_some() {
                osc.unbend();
            }
        }
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol;
    }

    pub fn set_atk(&mut self, atk: f32) {
        for osc in self.osc_s.iter_mut() {
            osc.env_filter.set_atk(atk);
        }
    }

    pub fn set_decay(&mut self, decay: f32) {
        for osc in self.osc_s.iter_mut() {
            osc.env_filter.set_decay(decay);
        }
    }

    pub fn set_sus(&mut self, sus: f32) {
        for osc in self.osc_s.iter_mut() {
            osc.env_filter.set_sus(sus);
        }
    }

    // pub fn set_release(&mut self, atk: f32) {
    //     for osc in self.osc_s.iter_mut() {
    //         osc.env_filter.set_re(atk);
    //     }
    // }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        let cutoff = cutoff * 10_000.0;

        for osc in self.osc_s.iter_mut() {
            osc.low_pass.set_cutoff(cutoff);
        }
    }

    pub fn set_resonace(&mut self, resonace: f32) {
        for osc in self.osc_s.iter_mut() {
            osc.low_pass.set_resonace(resonace);
        }
    }

    pub fn set_chorus_speed(&mut self, speed: f32) {
        self.chorus.set_speed(speed)
    }

    pub fn set_chorus_depth(&mut self, depth: f32) {
        self.chorus.set_volume(depth)
    }

    pub fn set_leslie_speed(&mut self, speed: f32) {
        self.lfo.set_frequency((400.0 * speed) / 60.0);
        self.lfo.set_volume(speed);
    }

    // pub fn set_atk(&mut self, atk: f32) {}
}
