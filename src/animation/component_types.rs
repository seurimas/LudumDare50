use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypeUuid};

use serde::{Deserialize, Serialize};

use super::data_types::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AnimationState {
    animation: String,
    pub timer: Timer,
    pub looping: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::looping("Idle".to_string())
    }
}

impl AnimationState {
    pub fn looping(animation: String) -> Self {
        Self::new(animation, true)
    }

    pub fn once(animation: String) -> Self {
        Self::new(animation, false)
    }

    pub fn new(animation: String, looping: bool) -> Self {
        AnimationState {
            animation,
            timer: Timer::from_seconds(3600.0, true),
            looping,
        }
    }

    pub fn get_animation(&self) -> &String {
        &self.animation
    }

    pub fn transition_to(&mut self, animation: &str, looping: bool) {
        if self.animation != animation {
            *self = Self::new(animation.to_string(), looping);
        }
    }

    pub fn try_transition_to(&mut self, animation: &str, looping: bool) -> bool {
        if self.get_animation().eq(animation) {
            false
        } else {
            self.transition_to(animation, looping);
            true
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Default, TypeUuid)]
#[uuid = "befc6683-46b1-4ce3-a364-d09c30c1c117"]
pub struct ParameterizedSpriteAnimationSet {
    animations: HashMap<String, ParameterizedSpriteAnimation>,
}

impl ParameterizedSpriteAnimationSet {
    pub fn len(&self) -> usize {
        self.animations.len()
    }

    pub fn animation_names(&self) -> impl Iterator<Item = &String> {
        self.animations.keys().into_iter()
    }

    pub fn get_animation(&self, name: &String) -> Option<&ParameterizedSpriteAnimation> {
        self.animations.get(name)
    }

    pub fn get_animation_mut(
        &mut self,
        name: &String,
    ) -> Option<&mut ParameterizedSpriteAnimation> {
        self.animations.get_mut(name)
    }

    pub fn add_new_animation(&mut self, name: String, animation: ParameterizedSpriteAnimation) {
        self.animations.insert(name, animation);
    }

    pub fn animation_complete(&self, animation_state: &AnimationState) -> bool {
        self.get_animation(animation_state.get_animation())
            .and_then(|animation| {
                animation.sample_with_parameter(
                    0,
                    animation_state.timer.elapsed_secs(),
                    animation_state.looping,
                )
            })
            .is_none()
    }
}
