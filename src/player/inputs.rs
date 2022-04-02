use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation::component_types::AnimationState, terrain::TerrainBlock};

use super::PlayerStats;

#[derive(Default, Component, Debug, Clone, Reflect, Inspectable)]
#[reflect(Component)]
pub struct PlayerInputState {
    tilt_x: f32,
    grounded: f32,
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

pub fn player_grounded_system(
    query_pipeline: Res<QueryPipeline>,
    time: Res<Time>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut player_query: Query<(&RigidBodyPositionComponent, &mut PlayerInputState)>,
    terrain_query: Query<&TerrainBlock>,
) {
    for (position, mut input) in player_query.iter_mut() {
        input.grounded -= time.delta_seconds();
        let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
        let shape = Cuboid::new(Vec2::new(1.0, 1.25).into());
        let shape_pos = position.0.position.translation.into();
        let groups = InteractionGroups::all();
        let filter = None;
        query_pipeline.intersections_with_shape(
            &collider_set,
            &shape_pos,
            &shape,
            groups,
            filter,
            |handle| {
                if terrain_query.get(handle.entity()).is_ok() {
                    input.grounded = 0.1;
                    false
                } else {
                    true
                }
            },
        )
    }
}

pub fn player_movement_system(
    mut query: Query<(
        &PlayerStats,
        &PlayerInputState,
        &mut TextureAtlasSprite,
        &mut AnimationState,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    for (stats, input, mut sprite, mut animation, mut velocity) in query.iter_mut() {
        if input.tilt_x != 0.0 {
            velocity.linvel.x = stats.walk_speed * input.tilt_x;
            animation.transition_to("Walk", true);
            sprite.flip_x = input.tilt_x < 0.0;
        } else {
            velocity.linvel.x = 0.0;
            animation.transition_to("Idle", true);
        }
        if velocity.linvel.y.abs() < 0.01 && input.wants_jump {
            velocity.linvel.y = stats.jump_speed;
        }
    }
}
