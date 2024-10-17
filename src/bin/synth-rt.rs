use anyhow::Result;
use midi_control::{KeyEvent, MidiMessage};
use rodio::OutputStream;
use serialport;
use std::{
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

    // start midi listener
    // spawn(move || {
    //     if let Err(e) = run_midi(synth) {
    //         println!("{e}");
    //         exit(1);
    //     }
    // });
    //
    // Ok(())

    run_midi(synth)
}

fn run_midi(synth: Arc<Mutex<Synth>>) -> Result<()> {
    loop {
        // read serial untill it can be read as midi
        let mut midi_cmd = [0; 5];
        let mut i = 0;
        let Some(Ok(port)) = glob::glob("/dev/ttyACM*")?.next() else {
            continue;
        };

        println!("port => {}", port.as_os_str().to_string_lossy());

        let mut serial_port =
            serialport::new(format!("{}", port.as_os_str().to_string_lossy()), 31_250)
                .timeout(Duration::from_millis(1000))
                .open()
                .expect("Failed to open serial port");

        while i < 5 {
            // read serial
            if let Err(e) = serial_port.read(&mut midi_cmd) {
                println!("{e}");
                break;
            };

            println!("{midi_cmd:?}");

            // parse into midi command

            let message = MidiMessage::from(midi_cmd.to_vec().as_ref());
            // do midi stuff

            match message {
                MidiMessage::Invalid => i += 1,
                MidiMessage::NoteOn(_, KeyEvent { key, value }) => {
                    synth.lock().unwrap().play(key, value);
                    break;
                }
                MidiMessage::NoteOff(_, KeyEvent { key, value: _ }) => {
                    synth.lock().unwrap().stop(key);
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
