use midi_control::MidiNote;

use crate::{
    lfo::LFO,
    osc::{Oscillator, Overtone},
};

pub const WAVE_TABLE_SIZE: usize = 256;
pub const VOICES: usize = 10;

pub struct Synth {
    osc_s: [Oscillator; VOICES],
    wave_table: [f32; WAVE_TABLE_SIZE],
    lfo: LFO,
    volume: f32,
}

impl Synth {
    pub fn new() -> Self {
        let overtones = [
            Some(Overtone {
                overtone: 1.0_f64.powf(1.0 / 12.0),
                volume: 1.0,
            }),
            Some(Overtone {
                // overtone: 2.0_f64.powf(1.0 / 12.0),
                overtone: 2.5_f64.powf(1.0 / 12.0),
                volume: 1.0,
            }),
            Some(Overtone {
                overtone: 2.0,
                volume: 1.0,
            }),
            Some(Overtone {
                overtone: 3.0,
                volume: 0.0 / 8.0,
            }),
            Some(Overtone {
                overtone: 4.0,
                volume: 0.0 / 8.0,
            }),
            Some(Overtone {
                overtone: 5.0,
                volume: 0.0 / 8.0,
            }),
            Some(Overtone {
                overtone: 7.0,
                volume: 0.0 / 8.0,
            }),
            Some(Overtone {
                overtone: 8.0,
                volume: 0.0 / 8.0,
            }),
            None,
            None,
        ];
        let wave_table = Self::build_wave_table(overtones);
        let mut lfo = LFO::new();
        lfo.set_frequency(100.0 / 60.0);

        Self {
            osc_s: [Oscillator::new(); VOICES],
            wave_table,
            lfo,
            volume: 0.75,
        }
    }

    fn build_wave_table(overtones: [Option<Overtone>; 10]) -> [f32; WAVE_TABLE_SIZE] {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let mut n_overtones = 0;

        for ot in overtones {
            if ot.is_some() {
                n_overtones += 1;
            }
        }

        let bias = 1.0 / n_overtones as f32;

        for i in 0..WAVE_TABLE_SIZE {
            for ot in overtones {
                if let Some(ot) = ot {
                    wave_table[i] += ((2.0 * core::f64::consts::PI * i as f64 * ot.overtone
                        / WAVE_TABLE_SIZE as f64)
                        .sin()
                        * ot.volume) as f32
                }
            }

            wave_table[i] *= bias;
        }

        wave_table
    }

    pub fn set_overtones(&mut self, overtones: [Option<Overtone>; 10]) {
        self.wave_table = Self::build_wave_table(overtones);
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
                sample += osc.get_sample(&self.wave_table);
                // println!(
                //     "env => {}, {}",
                //     osc.env_filter.get_samnple(),
                //     osc.env_filter.phase
                // );
            }
        }

        sample * (self.volume + lfo_sample * 0.125)
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

        for (i, osc) in self.osc_s.iter_mut().enumerate() {
            if osc.playing.is_none() {
                osc.press(midi_note);
                println!("playing note on osc {i}");

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
}
