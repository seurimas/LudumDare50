use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation::component_types::AnimationState, terrain::GroundedState};

use super::PlayerStats;

#[derive(Default, Component, Debug, Clone, Reflect, Inspectable)]
#[reflect(Component)]
pub struct PlayerInputState {
    tilt_x: f32,
    wants_attack: bool,
    wants_jump: bool,
}

pub fn player_key_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerInputState>,
) {
    for mut player in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            player.tilt_x = -1.0;
        } else if keyboard_input.pressed(KeyCode::D) {
            player.tilt_x = 1.0;
        } else {
            player.tilt_x = 0.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            player.wants_jump = true;
        } else {
            player.wants_jump = false;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            player.wants_attack = true;
        } else {
            player.wants_attack = false;
        }
    }
}

pub fn player_movement_system(
    mut query: Query<(
        &PlayerStats,
        &PlayerInputState,
        &mut GroundedState,
        &mut TextureAtlasSprite,
        &mut AnimationState,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    for (stats, input, mut grounded, mut sprite, mut animation, mut velocity) in query.iter_mut() {
        if grounded.on_the_ground() {
            if input.tilt_x != 0.0 {
                velocity.linvel.x = stats.walk_speed * input.tilt_x;
                animation.transition_to("Walk", true);
                sprite.flip_x = input.tilt_x < 0.0;
            } else {
                velocity.linvel.x = 0.0;
                animation.transition_to("Idle", true);
            }
            if input.wants_jump {
                velocity.linvel.y = stats.jump_speed;
                grounded.lift_off();
            }
        } else {
            if input.tilt_x != 0.0 {
                let desired_x_vel = stats.air_speed * input.tilt_x;
                if desired_x_vel > 0.0 && velocity.linvel.x < desired_x_vel {
                    velocity.linvel.x = desired_x_vel;
                    sprite.flip_x = input.tilt_x < 0.0;
                } else if desired_x_vel < 0.0 && velocity.linvel.x > desired_x_vel {
                    velocity.linvel.x = desired_x_vel;
                    sprite.flip_x = input.tilt_x < 0.0;
                }
            }
        }
    }
}
