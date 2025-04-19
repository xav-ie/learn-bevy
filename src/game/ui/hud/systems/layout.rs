use bevy::prelude::*;

use crate::game::ui::hud::{components::*, styles::*};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    if let Ok(hud) = hud_query.get_single() {
        commands.entity(hud).despawn_recursive();
    }
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    gap: Size::all(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Px(120.0),
                    },
                    ..default()
                },
                ..default()
            },
            Hud {},
        ))
        .with_children(|parent| {
            // Star Counter
            parent
                .spawn(NodeBundle {
                    style: NORMAL_HUD_ITEM_STYLE,
                    background_color: BackgroundColor(NORMAL_HUD_ITEM_COLOR),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::all(Val::Px(32.0)),
                            ..default()
                        },
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    normal_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        StarCounter {},
                    ));
                });

            // Enemy Counter
            parent
                .spawn(NodeBundle {
                    style: NORMAL_HUD_ITEM_STYLE,
                    background_color: BackgroundColor(NORMAL_HUD_ITEM_COLOR),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::all(Val::Px(32.0)),
                            ..default()
                        },
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "4",
                                    normal_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyCounter {},
                    ));
                });
        })
        .id();

    hud_entity
}
