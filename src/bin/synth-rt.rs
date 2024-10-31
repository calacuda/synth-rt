use anyhow::{bail, Result};
use iced::widget::{button, column, radio, row, text, vertical_slider, vertical_space, Column};
use iced::Alignment::Center;
use iced::{Element, Length};
use midi_control::{ControlEvent, KeyEvent, MidiMessage};
use rodio::OutputStream;
use serialport;
use std::{
    i16,
    io::{BufRead, BufReader},
    num::ParseIntError,
    process::exit,
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
    time::Duration,
};
use synth_rt::synth::OscType;
use synth_rt::{synth::Synth, Player};

pub struct SynthUI {
    synth: Arc<Mutex<Synth>>,
    _jhs: (JoinHandle<()>, JoinHandle<()>),
    _stream: OutputStream,
}

#[derive(Debug, Clone)]
enum Message {
    SetVolume(f32),
    OscVolume { osc_num: usize, vol: f32 },
    DetuneOscUp(usize),
    DetuneOscDown(usize),
    OscTypeUpdate { osc_num: usize, osc_type: OscType },
    ChorusVolume(f32),
    ChorusSpeed(f32),
}

impl SynthUI {
    /**
     * The title of the window. It will show up on the top of your application window.
     */
    fn title(&self) -> String {
        String::from("synth-rt")
    }

    fn update(&mut self, message: Message) {
        // Update the state of your app
        match message {
            Message::SetVolume(vol) => self.synth.lock().unwrap().set_volume(vol / 100.0),
            Message::OscVolume { osc_num, vol } => {
                self.synth.lock().unwrap().osc_type[osc_num].1 = vol / 100.0
            }
            Message::DetuneOscUp(osc_num) => {}
            Message::DetuneOscDown(osc_num) => {}
            Message::OscTypeUpdate { osc_num, osc_type } => {
                self.synth.lock().unwrap().osc_type[osc_num].0 = osc_type
            }
            Message::ChorusVolume(vol) => self.synth.lock().unwrap().chorus.set_volume(vol / 100.0),
            Message::ChorusSpeed(speed) => {
                self.synth.lock().unwrap().chorus.set_speed(speed / 100.0)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        println!("view");
        column![
            row![text!("waveform view").center()]
                .align_y(Center)
                .height(Length::FillPortion(20))
                .width(Length::Fill),
            row![
                column![text!("ADSR view").center()]
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(1)),
                column![text!("Low Pass view").center()]
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(1))
            ]
            .align_y(Center)
            .height(20)
            .width(Length::Fill),
            row![
                self.osc(0)
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(100)),
                self.osc(1)
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(100)),
                self.osc(2)
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(100)),
                self.overtones()
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(600)),
                column![
                    self.chorus(),
                    // column![text!("Reverb").size(24), text!("Decay"), text!("Gain")]
                    //     .align_x(Center)
                    //     .height(Length::FillPortion(50))
                    self.reverb()
                ]
                .align_x(Center)
                .height(Length::Fill)
                .width(Length::FillPortion(150)),
                column![self.vu_meter()]
                    .padding([24, 0])
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::FillPortion(150))
            ]
            .align_y(Center)
            .height(Length::FillPortion(60))
            .width(Length::Fill),
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        // .spacing(20)
        .align_x(Center)
        .into()
    }

    fn reverb(&self) -> Column<'_, Message> {
        // let decay = vertical_slider(
        //     0.0..=100.0,
        //     self.synth.lock().unwrap().reverb.decay * 100.0,
        //     Message::ChorusVolume,
        // );
        //
        // let gain = vertical_slider(
        //     0.0..=100.0,
        //     self.synth.lock().unwrap().reverb.gain * 100.0,
        //     Message::ChorusSpeed,
        // );

        column![
            text!["Reverb"].size(24),
            text!["Decay"],
            // decay,
            text!["Gain"],
            // gain
        ]
        .align_x(Center)
        .height(Length::FillPortion(50))
        .into()
    }

    fn chorus(&self) -> Column<'_, Message> {
        let volume = vertical_slider(
            0.0..=100.0,
            self.synth.lock().unwrap().chorus.volume * 100.0,
            Message::ChorusVolume,
        );

        let speed = vertical_slider(
            0.0..=100.0,
            self.synth.lock().unwrap().chorus.speed * 100.0,
            Message::ChorusSpeed,
        );

        column![
            text!["Chorus"].size(24),
            text!["Vol."],
            volume,
            text!["Speed"],
            speed
        ]
        .align_x(Center)
        .height(Length::FillPortion(50))
        .into()
    }

    fn osc(&self, osc_i: usize) -> Column<'_, Message> {
        let volume = vertical_slider(
            0.0..=100.0,
            self.synth.lock().unwrap().osc_type[osc_i].1 * 100.0,
            move |vol| Message::OscVolume {
                osc_num: osc_i,
                vol,
            },
        );

        let detune = column![
            // detune up
            button("Up")
                // .padding([12, 24])
                .on_press(Message::DetuneOscUp(osc_i)),
            vertical_space(),
            // detune amt
            // text!("Detune: {}", 0),
            vertical_space(),
            // detune down
            button("Dwn")
                // .padding([12, 24])
                .on_press(Message::DetuneOscDown(osc_i)),
            // horizontal_space(),
        ];

        let selection = Some(self.synth.lock().unwrap().osc_type[osc_i].0);

        let waveform = column![
            text!["WaveForm"],
            radio("Sin", OscType::Sin, selection, |osc_type| {
                Message::OscTypeUpdate {
                    osc_num: osc_i,
                    osc_type,
                }
            },),
            radio("Tri", OscType::Tri, selection, |osc_type| {
                Message::OscTypeUpdate {
                    osc_num: osc_i,
                    osc_type,
                }
            },),
            radio("Saw", OscType::Saw, selection, |osc_type| {
                Message::OscTypeUpdate {
                    osc_num: osc_i,
                    osc_type,
                }
            },),
            radio("Sqr", OscType::Sqr, selection, |osc_type| {
                Message::OscTypeUpdate {
                    osc_num: osc_i,
                    osc_type,
                }
            },),
        ];

        column![
            text!("Osc {}", osc_i + 1)
                .size(24)
                .align_x(Center)
                .align_y(Center)
                .height(Length::FillPortion(10))
                .width(Length::Fill),
            column![text!("Vol.").center(), volume]
                .align_x(Center)
                .height(Length::FillPortion(30))
                .width(Length::Fill),
            column![text!("Detune: {}", 0).center(), detune.align_x(Center)]
                .align_x(Center)
                .height(Length::FillPortion(30))
                .width(Length::Fill),
            waveform
                .align_x(Center)
                .height(Length::FillPortion(30))
                .width(Length::Fill),
        ]
        .align_x(Center)
        .height(Length::Fill)
        .width(Length::FillPortion(1))
        .into()
    }

    fn overtones(&self) -> Column<'_, Message> {
        let overtones: Vec<Element<Message>> = (0..10)
            .map(|i| {
                column![text!("Overtones {}", i + 1).size(12).center()]
                    .width(Length::FillPortion(10))
                    .into()
            })
            .collect();

        column![
            row![text!("Overtones").size(24).center()]
                .align_y(Center)
                .height(Length::FillPortion(10))
                .width(Length::Fill),
            row(overtones)
                .align_y(Center)
                .height(Length::FillPortion(90))
                .width(Length::Fill)
        ]
        .align_x(Center)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }

    fn vu_meter(&self) -> Element<Message> {
        vertical_slider(
            0.0..=100.0,
            self.synth.lock().unwrap().volume * 100.0,
            Message::SetVolume,
        )
        .into()
    }
}

impl Default for SynthUI {
    fn default() -> Self {
        let synth = {
            let synth = Synth::new();
            // println!("synth volume => {}", synth.volume);
            Arc::new(Mutex::new(synth))
        };

        let output = Player {
            synth: synth.clone(),
        };
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // start output
        let jh_1 = spawn(move || {
            if let Err(e) = stream_handle.play_raw(output) {
                println!("[ERROR] => audio playback error: {e}");
                exit(1);
            }
        });

        let s = synth.clone();
        let jh_2 = spawn(move || {
            if let Err(e) = run_midi(s) {
                println!("[ERROR] => Serial MIDI input error: {e}");
                exit(1);
            }
        });
        let _jhs = (jh_1, jh_2);

        Self {
            synth,
            _jhs,
            _stream,
        }
    }
}

fn main() -> iced::Result {
    iced::application(SynthUI::title, SynthUI::update, SynthUI::view)
        .centered()
        .run()
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

    let serial_port = serialport::new(format!("{}", port.as_os_str().to_string_lossy()), 32_500)
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
                // println!("midi message => {message:?}");
                // do midi stuff

                match message {
                    MidiMessage::Invalid => {
                        // println!("midi_cmd_buf => {midi_cmd:?}");
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
            });
        } else {
            println!("bad HEX");
        }
    }
}
