use bevy::prelude::*;

use crate::game::{
    enemy::components::Enemy,
    score::resources::Score,
    ui::hud::components::{EnemyCounter, StarCounter},
};

pub fn update_score_text(mut text_query: Query<&mut Text, With<StarCounter>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyCounter>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", count.to_string());
    }
}
