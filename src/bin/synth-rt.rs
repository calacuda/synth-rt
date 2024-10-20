use anyhow::{bail, Result};
use midi_control::{ControlEvent, KeyEvent, MidiMessage};
use rodio::OutputStream;
use serialport;
use std::{
    i16,
    io::{BufRead, BufReader},
    num::ParseIntError,
    process::exit,
    sync::{Arc, Mutex},
    thread::spawn,
    time::Duration,
};
use synth_rt::{synth::Synth, Player};

fn main() -> Result<()> {
    // build synth in arc mutex
    let synth = Arc::new(Mutex::new(Synth::new()));

    let output = Player {
        synth: synth.clone(),
    };
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // start output
    spawn(move || {
        if let Err(e) = stream_handle.play_raw(output) {
            println!("[ERROR] => {e}");
            exit(1);
        }
    });

    run_midi(synth)
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.replace("\r\n", "");
    // println!("{s:#?}");

    let bytes: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();

    Ok(bytes?)
}

fn run_midi(synth: Arc<Mutex<Synth>>) -> Result<()> {
    let Some(Ok(port)) = glob::glob("/dev/ttyACM*")?.next() else {
        // continue;
        bail!("no serial ports found");
    };

    let mut serial_port =
        serialport::new(format!("{}", port.as_os_str().to_string_lossy()), 32_500)
            .timeout(Duration::from_millis(u64::MAX))
            .open()?;

    let mut reader = BufReader::new(serial_port);

    loop {
        // read serial untill it can be read as midi
        let mut midi_cmd = String::with_capacity(10);

        if let Err(e) = reader.read_line(&mut midi_cmd) {
            println!("{e}");
            continue;
        }

        // parse into midi command
        let synth = synth.clone();

        if let Ok(midi_cmd) = decode_hex(&midi_cmd) {
            spawn(move || {
                let message = MidiMessage::from(midi_cmd.as_ref());
                // do midi stuff

                match message {
                    MidiMessage::Invalid => {
                        println!("midi_cmd_buf => {midi_cmd:?}");
                        println!("midi cmd => {:?}", MidiMessage::from(midi_cmd.as_ref()));
                        println!("midi_cmd -> {midi_cmd:?}");
                        println!("midi command invalid");
                    }
                    MidiMessage::NoteOn(_, KeyEvent { key, value }) => {
                        synth.lock().unwrap().play(key, value)
                    }
                    MidiMessage::NoteOff(_, KeyEvent { key, value: _ }) => {
                        synth.lock().unwrap().stop(key)
                    }
                    MidiMessage::PitchBend(_, lsb, msb) => {
                        let bend = i16::from_le_bytes([lsb, msb]) as f32 / (32_000.0 * 0.5) - 1.0;

                        if bend > 0.026 || bend < -0.026 {
                            synth.lock().unwrap().bend_all(bend);
                        } else {
                            synth.lock().unwrap().unbend();
                        }
                    }
                    MidiMessage::ControlChange(_, ControlEvent { control, value }) => {
                        let value = value as f32 / 127.0;

                        match control {
                            70 => synth.lock().unwrap().set_volume(value),
                            71 => synth.lock().unwrap().set_atk(value),
                            72 => synth.lock().unwrap().set_decay(value),
                            73 => synth.lock().unwrap().set_sus(value),
                            // 74 => synth.lock().unwrap().set_cutoff(value),
                            1 => synth.lock().unwrap().set_cutoff(value),
                            74 => synth.lock().unwrap().set_resonace(value),
                            // 75 => synth.lock().unwrap().set_chorus_depth(value),
                            // 76 => synth.lock().unwrap().set_chorus_speed(value),
                            77 => synth.lock().unwrap().set_leslie_speed(value),
                            _ => {}
                        }
                    }
                    _ => {} // }
                }
            });
        } else {
            println!("bad HEX");
        }
    }
}
