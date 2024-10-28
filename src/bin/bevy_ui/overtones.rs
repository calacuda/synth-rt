use bevy::prelude::*;

pub fn spawn_overtones(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
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
                    "Overtones",
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
                // border: UiRect::all(Val::Px(8.)),
                height: Val::Percent(90.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            // background_color: Color::srgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder<'_>| {
            for i in 0..10 {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(10.0),
                            border: UiRect::all(Val::Px(8.)),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                        ..default()
                    })
                    .with_children(|parent: &mut ChildBuilder<'_>| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                format!("OT {}", i),
                                TextStyle {
                                    font: asset_server.load("fonts/Anonymous Pro B.ttf"),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}
