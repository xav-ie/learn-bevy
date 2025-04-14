use bevy::prelude::*;

use crate::game::ui::pause_menu::components::*;
use crate::styles::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu) = pause_menu_query.get_single() {
        commands.entity(pause_menu).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    gap: Size::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    position: UiRect::all(Val::Px(0.0)),
                    position_type: PositionType::Absolute,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
                    ..default()
                },
                background_color: Color::rgba(0.15, 0.15, 0.15, 0.85).into(),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            // Title text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "PAUSED",
                        TextStyle {
                            font_size: 84.0,
                            ..normal_text_style(asset_server)
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                style: Style {
                    padding: UiRect::all(Val::Px(80.0)),
                    ..default()
                },
                ..default()
            });

            // Resume button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                        ..default()
                    },
                    ResumeButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Resume",
                                normal_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // Main menu button
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                        ..default()
                    },
                    MainMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Main Menu",
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

    pause_menu_entity
}
