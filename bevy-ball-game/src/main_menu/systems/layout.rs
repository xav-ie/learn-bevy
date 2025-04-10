use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu) = main_menu_query.get_single() {
        commands.entity(main_menu).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(300.0), Val::Px(120.0)),
                        gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // image
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                            ..default()
                        },
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });

                    // text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Ball Game",
                                TextStyle {
                                    font_size: 64.0,
                                    ..normal_text_style(asset_server)
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    // image
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                            ..default()
                        },
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });

            // Play button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                normal_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // Quit button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                normal_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
