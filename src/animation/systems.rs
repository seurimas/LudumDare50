use bevy::prelude::*;

use super::component_types::{AnimationState, ParameterizedSpriteAnimationSet};

pub fn animation_timer_system(time: Res<Time>, mut query: Query<&mut AnimationState>) {
    query.for_each_mut(|mut animation_state| {
        animation_state.timer.tick(time.delta());
    });
}

pub fn direction_parameter_animation_system(
    mut query: Query<(
        &AnimationState,
        &mut TextureAtlasSprite,
        &ParameterizedSpriteAnimationSet,
    )>,
) {
    query.for_each_mut(|(animation_state, mut sprite, animation_set)| {
        if let Some(animation) = animation_set.get_animation(animation_state.get_animation()) {
            if let Some(new_index) = animation.sample_with_parameter(
                0,
                animation_state.timer.elapsed_secs(),
                animation_state.looping,
            ) {
                sprite.index = new_index;
            }
        }
    })
}
