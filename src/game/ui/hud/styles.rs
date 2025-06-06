use crate::styles::normal_text_style as normal_text_style_super;
use bevy::prelude::*;

pub const NORMAL_HUD_ITEM_COLOR: Color = Color::rgba(0.15, 0.15, 0.15, 0.5);

pub const NORMAL_HUD_ITEM_SIZE: Size = Size::new(Val::Px(150.0), Val::Percent(100.0));
pub const NORMAL_HUD_ITEM_STYLE: Style = Style {
    size: NORMAL_HUD_ITEM_SIZE,
    display: Display::Flex,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    gap: Size::all(Val::Px(8.0)),
    ..Style::DEFAULT
};

pub fn normal_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font_size: 52.0,
        ..normal_text_style_super(asset_server)
    }
}
