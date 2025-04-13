use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const ACTIVE_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub const NORMAL_BUTTON_SIZE: Size = Size::all(Val::Auto);
pub const NORMAL_BUTTON_STYLE: Style = Style {
    size: NORMAL_BUTTON_SIZE,
    display: Display::Flex,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    gap: Size::all(Val::Px(8.0)),
    padding: UiRect::all(Val::Px(32.0)),
    ..Style::DEFAULT
};

pub fn normal_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
