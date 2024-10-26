use anyhow::{bail, Result};
use bevy::{
    color::palettes::css::*,
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};
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
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_translation, animate_rotation, animate_scale),
        )
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };
    let text_justification = JustifyText::Center;
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    // Demonstrate changing translation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translation", text_style.clone())
                .with_justify(text_justification),
            ..default()
        },
        AnimateTranslation,
    ));
    // Demonstrate changing rotation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("rotation", text_style.clone())
                .with_justify(text_justification),
            ..default()
        },
        AnimateRotation,
    ));
    // Demonstrate changing scale
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("scale", text_style).with_justify(text_justification),
            transform: Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
            ..default()
        },
        AnimateScale,
    ));
    // Demonstrate text wrapping
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 42.0,
        ..default()
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    justify: JustifyText::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.20, 0.3, 0.70),
                custom_size: Some(Vec2::new(other_box_size.x, other_box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(other_box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(AnyCharacter linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    justify: JustifyText::Left,
                    linebreak_behavior: BreakLineOn::AnyCharacter,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: other_box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    for (text_anchor, color) in [
        (Anchor::TopLeft, Color::Srgba(RED)),
        (Anchor::TopRight, Color::Srgba(LIME)),
        (Anchor::BottomRight, Color::Srgba(BLUE)),
        (Anchor::BottomLeft, Color::Srgba(YELLOW)),
    ] {
        commands.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!(" Anchor::{text_anchor:?} "),
                    TextStyle {
                        color,
                        ..slightly_smaller_text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(250. * Vec3::Y),
            text_anchor,
            ..default()
        });
    }
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        let scale = (time.elapsed_seconds().sin() + 1.1) * 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
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
