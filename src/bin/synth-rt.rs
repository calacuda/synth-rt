use anyhow::{bail, Result};
use iced::widget::{
    button, column, radio, row, svg, text, vertical_slider, vertical_space, Column, Row,
};
use iced::Alignment::Center;
use iced::{Element, Length, Padding};
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
use synth_rt::synth::{OscType, WAVE_TABLE_SIZE};
use synth_rt::{synth::Synth, Player};

pub struct SynthUI {
    synth: Arc<Mutex<Synth>>,
    jhs: (JoinHandle<()>, JoinHandle<()>),
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
    ConnectToSerial,
    ReverbGain(f32),
    ReverbDecay(f32),
    OvertoneVolume { overtone: usize, vol: f64 },
}

impl SynthUI {
    /// The title of the window. It will show up on the top of your application window.
    fn title(&self) -> String {
        String::from("synth-rt")
    }

    /// Updated the state of your app
    fn update(&mut self, message: Message) {
        match message {
            Message::SetVolume(vol) => self.synth.lock().unwrap().set_volume(vol / 100.0),
            Message::OscVolume { osc_num, vol } => {
                self.synth.lock().unwrap().osc_type[osc_num].1 = vol / 100.0
            }
            Message::DetuneOscUp(osc_num) => {
                if self.synth.lock().unwrap().osc_s[osc_num].1 < 12 {
                    self.synth.lock().unwrap().osc_s[osc_num].1 += 1
                }
            }
            Message::DetuneOscDown(osc_num) => {
                if self.synth.lock().unwrap().osc_s[osc_num].1 > -12 {
                    self.synth.lock().unwrap().osc_s[osc_num].1 -= 1
                }
            }
            Message::OscTypeUpdate { osc_num, osc_type } => {
                self.synth.lock().unwrap().osc_type[osc_num].0 = osc_type
            }
            Message::ChorusVolume(vol) => self.synth.lock().unwrap().chorus.set_volume(vol / 100.0),
            Message::ChorusSpeed(speed) => {
                self.synth.lock().unwrap().chorus.set_speed(speed / 100.0)
            }
            Message::ConnectToSerial => {
                let s = self.synth.clone();
                self.jhs.1 = spawn(move || con_to_serial(s))
            }
            Message::ReverbGain(_) | Message::ReverbDecay(_) => {
                println!("[ERROR] Reverb is not yet implemented")
            }
            Message::OvertoneVolume { overtone, vol } => {
                self.synth.lock().unwrap().overtones[overtone].volume = vol / 100.0;
                self.synth.lock().unwrap().set_overtones();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if !self.jhs.1.is_finished() {
            self.synth_view()
        } else {
            self.con_serial_view()
        }
    }

    fn con_serial_view(&self) -> Element<Message> {
        let con_button = button("Connect").on_press(Message::ConnectToSerial);

        column![text!["no serial MIDI connection found. Please connect the arduino and click the button bellow."].size(24), con_button]
            .height(Length::Fill)
            .width(Length::Fill)
            // .spacing(20)
            .align_x(Center)
            .into()
    }

    /// the main layout for the app when serial is connected
    fn synth_view(&self) -> Element<Message> {
        // println!("view");
        column![
            // row![text!("waveform view").center()]
            self.waveform_vis()
                .align_y(Center)
                .height(Length::FillPortion(40))
                .width(Length::Fill),
            // row![
            //     column![text!("ADSR view").center()]
            //         .align_x(Center)
            //         .height(Length::Fill)
            //         .width(Length::FillPortion(1)),
            //     column![text!("Low Pass view").center()]
            //         .align_x(Center)
            //         .height(Length::Fill)
            //         .width(Length::FillPortion(1))
            // ]
            // .align_y(Center)
            // .height(20)
            // .width(Length::Fill),
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

    fn waveform_vis(&self) -> Row<'_, Message> {
        let waveforms = self.synth.lock().unwrap().wave_tables.clone();
        let weights = self.synth.lock().unwrap().osc_type.clone();

        let mut waveform = [0.0; WAVE_TABLE_SIZE];

        for (osc_type, weight) in weights {
            let wf = match osc_type {
                OscType::Sin => waveforms.sin.clone(),
                OscType::Tri => waveforms.tri.clone(),
                OscType::Sqr => waveforms.sqr.clone(),
                OscType::Saw => waveforms.saw.clone(),
            };

            wf.iter()
                .enumerate()
                .for_each(|(i, sample)| waveform[i] += (sample * weight) / 3.0)
        }

        let waveform = waveform.into_iter().enumerate();

        let mut graph = Vec::with_capacity(WAVE_TABLE_SIZE + 2);
        graph.push(format!("<?xml version=\"1.0\" standalone=\"no\"?>\n<svg id=\"waveform-graph\"  height=\"100%\" width=\"100%\" viewBox=\"0 0 {} 50\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">", WAVE_TABLE_SIZE * 2));

        for ((x1, s1), (x2, s2)) in waveform.clone().zip(waveform.skip(1)) {
            let y1 = s1 * 25.0 + 25.0;
            let y2 = s2 * 25.0 + 25.0;
            let x1 = (x1 + 1) * 2;
            let x2 = (x2 + 1) * 2;
            graph.push(format!("<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" style=\"stroke:blue;stroke-width:1\"/>"));
        }

        graph.push("</svg>".into());
        // println!("{}", graph.join(" "));

        let graph_svg = graph.join(" ").as_bytes().to_vec();
        let handle = svg::Handle::from_memory(graph_svg);

        row![svg(handle).width(Length::Fill).height(Length::Fill)]
    }

    fn reverb(&self) -> Column<'_, Message> {
        let decay = vertical_slider(
            0.0..=100.0,
            0.0, // self.synth.lock().unwrap().reverb.decay * 100.0,
            Message::ReverbDecay,
        );

        let gain = vertical_slider(
            0.0..=100.0,
            0.0, // self.synth.lock().unwrap().reverb.gain * 100.0,
            Message::ReverbGain,
        );

        column![
            text!["Reverb"].size(24),
            row![
                column![text!["Decay"], decay,]
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::Fill),
                column![text!["Gain"], gain]
                    // .padding([24, 0])
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::Fill),
            ]
            .padding(Padding {
                bottom: 24.0,
                ..Default::default()
            })
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
            row![
                column![text!["Vol."], volume]
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::Fill),
                column![text!["Speed"], speed]
                    .align_x(Center)
                    .height(Length::Fill)
                    .width(Length::Fill)
            ]
            .align_y(Center)
            .height(Length::Fill)
            .width(Length::Fill) // .into()
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

        let detune_amt = self.synth.lock().unwrap().osc_s[osc_i].1;

        let detune = column![
            // detune up
            button("Up")
                // .padding([12, 24])
                .on_press(Message::DetuneOscUp(osc_i)),
            vertical_space(),
            // detune amt
            text!("{}", detune_amt),
            // vertical_space(),
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
            column![text!("Detune").center(), detune.align_x(Center)]
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
                let slider = vertical_slider(
                    0.0..=100.0,
                    self.synth.lock().unwrap().overtones[i].volume * 100.0,
                    move |vol| Message::OvertoneVolume { overtone: i, vol },
                );
                column![text!("{}", i + 1).center(), slider]
                    .padding([24, 0])
                    .width(Length::FillPortion(10))
                    .into()
            })
            .collect();

        column![
            text!("Overtones")
                .size(24)
                .center()
                .align_y(Center)
                .align_x(Center)
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
        let jh_2 = spawn(move || con_to_serial(s));
        let jhs = (jh_1, jh_2);

        Self {
            synth,
            jhs,
            _stream,
        }
    }
}

fn main() -> iced::Result {
    iced::application(SynthUI::title, SynthUI::update, SynthUI::view)
        .centered()
        .run()
}

fn con_to_serial(s: Arc<Mutex<Synth>>) {
    if let Err(e) = run_midi(s) {
        println!("[ERROR] => Serial MIDI input error: {e}");
        // exit(1);
    }
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
                            // 70 => synth.lock().unwrap().set_volume(value),
                            70 => synth.lock().unwrap().set_atk(value),
                            71 => synth.lock().unwrap().set_decay(value),
                            72 => synth.lock().unwrap().set_sus(value),
                            73 => synth.lock().unwrap().set_release(value),
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
