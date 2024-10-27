use anyhow::{bail, Result};
use bevy::{
    color::palettes::css::*,
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
    winit::WinitSettings,
};
use bevy_framepace::{FramepaceSettings, Limiter};
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

#[derive(Resource)]
pub struct SynthMarker(Arc<Mutex<Synth>>);

fn main() {
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

    // run_midi(synth)
    App::new()
        .insert_resource(SynthMarker(synth))
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, set_framerate)
        .add_systems(Startup, setup)
        // .add_systems(
        //     Update,
        //     (animate_translation, animate_rotation, animate_scale),
        // )
        .run();
}

fn set_framerate(mut frame_settings: ResMut<FramepaceSettings>) {
    frame_settings.limiter = Limiter::from_framerate(20.0);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(5.)),
                                row_gap: Val::Px(5.),
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                        font_size: 30.0,
                                        ..default()
                                    },
                                ),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));

                            #[cfg(feature = "bevy_dev_tools")]
                            // Debug overlay text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Press Space to enable debug outlines.",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        ..default()
                                    },
                                ),
                                Label,
                            ));

                            #[cfg(not(feature = "bevy_dev_tools"))]
                            parent.spawn((
                                TextBundle::from_section(
                                    "Try enabling feature \"bevy_dev_tools\".",
                                    TextStyle {
                                        font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                        ..default()
                                    },
                                ),
                                Label,
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            height: Val::Percent(50.),
                            overflow: Overflow::clip_y(),
                            ..default()
                        },
                        background_color: Color::srgb(0.10, 0.10, 0.10).into(),
                        ..default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(210.),
                        bottom: Val::Px(10.),
                        border: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    border_color: LIME.into(),
                    background_color: Color::srgb(0.4, 0.4, 1.).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.8, 0.8, 1.).into(),
                        ..default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(1.0, 0.0, 0.).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    // Take the size of the parent node.
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(20.),
                                    bottom: Val::Px(20.),
                                    ..default()
                                },
                                background_color: Color::srgb(1.0, 0.3, 0.3).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(40.),
                                    bottom: Val::Px(40.),
                                    ..default()
                                },
                                background_color: Color::srgb(1.0, 0.5, 0.5).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(60.),
                                    bottom: Val::Px(60.),
                                    ..default()
                                },
                                background_color: Color::srgb(1.0, 0.7, 0.7).into(),
                                ..default()
                            });
                            // alpha test
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(80.),
                                    bottom: Val::Px(80.),
                                    ..default()
                                },
                                background_color: Color::srgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    // A `NodeBundle` is used to display the logo the image as an `ImageBundle` can't automatically
                    // size itself with a child node present.
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(500.0),
                                    height: Val::Px(125.0),
                                    margin: UiRect::top(Val::VMin(5.)),
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(asset_server.load("branding/bevy_logo_dark_big.png")),
                        ))
                        .with_children(|parent| {
                            // alt text
                            // This UI node takes up no space in the layout and the `Text` component is used by the accessibility module
                            // and is not rendered.
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                Text::from_section("Bevy logo", TextStyle::default()),
                            ));
                        });
                });
        });
}

// pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
//     let s = s.replace("\r\n", "");
//     // println!("{s:#?}");
//
//     let bytes: Result<Vec<u8>, ParseIntError> = (0..s.len())
//         .step_by(2)
//         .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
//         .collect();
//
//     Ok(bytes?)
// }
//
// fn run_midi(synth: Arc<Mutex<Synth>>) -> Result<()> {
//     let Some(Ok(port)) = glob::glob("/dev/ttyACM*")?.next() else {
//         // continue;
//         bail!("no serial ports found");
//     };
//
//     let mut serial_port =
//         serialport::new(format!("{}", port.as_os_str().to_string_lossy()), 32_500)
//             .timeout(Duration::from_millis(u64::MAX))
//             .open()?;
//
//     let mut reader = BufReader::new(serial_port);
//
//     loop {
//         // read serial untill it can be read as midi
//         let mut midi_cmd = String::with_capacity(10);
//
//         if let Err(e) = reader.read_line(&mut midi_cmd) {
//             println!("{e}");
//             continue;
//         }
//
//         // parse into midi command
//         let synth = synth.clone();
//
//         if let Ok(midi_cmd) = decode_hex(&midi_cmd) {
//             spawn(move || {
//                 let message = MidiMessage::from(midi_cmd.as_ref());
//                 // do midi stuff
//
//                 match message {
//                     MidiMessage::Invalid => {
//                         println!("midi_cmd_buf => {midi_cmd:?}");
//                         println!("midi cmd => {:?}", MidiMessage::from(midi_cmd.as_ref()));
//                         println!("midi_cmd -> {midi_cmd:?}");
//                         println!("midi command invalid");
//                     }
//                     MidiMessage::NoteOn(_, KeyEvent { key, value }) => {
//                         synth.lock().unwrap().play(key, value)
//                     }
//                     MidiMessage::NoteOff(_, KeyEvent { key, value: _ }) => {
//                         synth.lock().unwrap().stop(key)
//                     }
//                     MidiMessage::PitchBend(_, lsb, msb) => {
//                         let bend = i16::from_le_bytes([lsb, msb]) as f32 / (32_000.0 * 0.5) - 1.0;
//
//                         if bend > 0.026 || bend < -0.026 {
//                             synth.lock().unwrap().bend_all(bend);
//                         } else {
//                             synth.lock().unwrap().unbend();
//                         }
//                     }
//                     MidiMessage::ControlChange(_, ControlEvent { control, value }) => {
//                         let value = value as f32 / 127.0;
//
//                         match control {
//                             70 => synth.lock().unwrap().set_volume(value),
//                             71 => synth.lock().unwrap().set_atk(value),
//                             72 => synth.lock().unwrap().set_decay(value),
//                             73 => synth.lock().unwrap().set_sus(value),
//                             74 => synth.lock().unwrap().set_cutoff(value),
//                             75 => synth.lock().unwrap().set_resonace(value),
//                             76 => synth.lock().unwrap().set_chorus_depth(value),
//                             77 => synth.lock().unwrap().set_chorus_speed(value),
//                             1 => synth.lock().unwrap().set_leslie_speed(value),
//                             _ => {}
//                         }
//                     }
//                     _ => {} // }
//                 }
//             });
//         } else {
//             println!("bad HEX");
//         }
//     }
// }
