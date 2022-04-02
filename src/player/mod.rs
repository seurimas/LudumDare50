mod camera;
mod inputs;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::RegisterInspectable;
use bevy_rapier2d::physics::RigidBodyPositionSync;
use bevy_rapier2d::render::ColliderDebugRender;

use crate::animation::bundles::AnimatedSprite;
use crate::base_bundles::WorldEntityBuilder;
use crate::setup_camera;

use self::camera::player_camera_system;
use self::inputs::player_grounded_system;
use self::inputs::player_key_input_system;
use self::inputs::player_movement_system;
use self::inputs::PlayerInputState;

#[derive(Component, Debug, Reflect, Inspectable, Default, Copy, Clone)]
pub struct PlayerStats {
    pub walk_speed: f32,
    pub jump_speed: f32,
    pub jump_delay: f32,
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let world_entity = WorldEntityBuilder::of_size(1.0).at_position(0.0, 10.0);
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    transform.scale = Vec3::new(1.0 / 6.4, 1.0 / 6.4, 1.0 / 6.4);
    commands
        .spawn_bundle(AnimatedSprite {
            sprite_animation: assets.load("sprites/Player.sprite"),
            transform,
            ..Default::default()
        })
        .insert_bundle(world_entity.rigid_body_bundle())
        .insert_bundle(world_entity.collider_bundle())
        .insert(PlayerInputState::default())
        .insert(PlayerStats {
            walk_speed: 10.0,
            jump_speed: 50.0,
            jump_delay: 0.15,
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("player"));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_camera_system)
            .add_system(player_movement_system)
            .add_system(player_key_input_system)
            .add_system(player_grounded_system)
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_player)
            .register_type::<PlayerStats>()
            .register_inspectable::<PlayerStats>()
            .register_type::<PlayerInputState>()
            .register_inspectable::<PlayerInputState>();
    }
}
