use anyhow::{bail, Result};
use midi_control::{ControlEvent, KeyEvent, MidiMessage};
use midir::{Ignore, MidiInput};
use rodio::OutputStream;
use serialport;
use std::{
    i16,
    io::{BufRead, BufReader},
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

fn run_midi(synth: Arc<Mutex<Synth>>) -> Result<()> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    println!("midi in ports len => {}", in_ports.len());
    // let in_port = match in_ports.len() {
    //     0 => bail!("no input port found"),
    //     1 => {
    //         println!(
    //             "Choosing the only available input port: {}",
    //             midi_in.port_name(&in_ports[0]).unwrap()
    //         );
    //         &in_ports[0]
    //     }
    //     _ => bail!("too many input ports found"),
    // };

    println!("\nOpening connections");
    // let in_port_name = midi_in.port_name(in_port)?;
    let mut connections = Vec::with_capacity(in_ports.len());

    for in_port in in_ports.iter() {
        let mut midi_in = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);
        let synth = synth.clone();

        // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
        connections.push(midi_in.connect(
            in_port,
            "midir-read-input",
            move |_stamp, message, _| {
                // println!("{}: {:?} (len = {})", stamp, message, message.len());
                let message = MidiMessage::from(message);
                // do midi stuff

                match message {
                    MidiMessage::Invalid => {
                        println!("midi_cmd_buf => {message:?}");
                        println!("midi_cmd -> {message:?}");
                        println!("midi cmd => {:?}", MidiMessage::from(message));
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
                            74 => synth.lock().unwrap().set_cutoff(value),
                            75 => synth.lock().unwrap().set_resonace(value),
                            76 => synth.lock().unwrap().set_chorus_depth(value),
                            77 => synth.lock().unwrap().set_chorus_speed(value),
                            1 => synth.lock().unwrap().set_leslie_speed(value),
                            _ => {}
                        }
                    }
                    _ => {} // }
                }
            },
            (),
        ))
    }

    loop {}
}
