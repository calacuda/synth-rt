use bevy::prelude::*;

pub fn spawn_osc_ctrl(
    parent: &mut ChildBuilder<'_>,
    osc_num: usize,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(8.)),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("OSC {}", osc_num + 1),
                    TextStyle {
                        font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        });
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(8.)),
                height: Val::Percent(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Vol.",
                    TextStyle {
                        font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                        font_size: 24.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(8.)),
                height: Val::Percent(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Detune.",
                    TextStyle {
                        font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                        font_size: 24.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(8.)),
                height: Val::Percent(30.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent
                .spawn(ButtonBundle {
                    background_color: Color::srgb(0.0, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Sin",
                            TextStyle {
                                font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                font_size: 24.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
            parent
                .spawn(ButtonBundle {
                    background_color: Color::srgb(0.0, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Tri",
                            TextStyle {
                                font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                font_size: 24.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
            parent
                .spawn(ButtonBundle {
                    background_color: Color::srgb(0.0, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Saw",
                            TextStyle {
                                font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                font_size: 24.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
            parent
                .spawn(ButtonBundle {
                    background_color: Color::srgb(0.0, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Sqr",
                            TextStyle {
                                font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                font_size: 24.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
        });
}
