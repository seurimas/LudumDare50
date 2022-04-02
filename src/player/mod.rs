mod attack_behavior_tree;
mod camera;
mod combat;
mod inputs;

use crate::prelude::*;

use crate::animation::bundles::AnimatedSprite;
use crate::base_bundles::WorldEntityBuilder;
use crate::setup_camera;
use crate::terrain::GroundedState;

use self::attack_behavior_tree::attack_brain_system;
use self::attack_behavior_tree::attack_impulse_system;
use self::attack_behavior_tree::attack_impulse_update_system;
use self::attack_behavior_tree::AttackImpulses;
use self::attack_behavior_tree::PlayerAttackType;
use self::camera::player_camera_system;
use self::combat::player_hit_stun_recovery_system;
use self::inputs::player_key_input_system;
use self::inputs::player_movement_system;
use self::inputs::PlayerInputState;

#[derive(Component, Debug, Reflect, Inspectable, Default, Copy, Clone)]
pub struct PlayerStats {
    pub walk_speed: f32,
    pub air_speed: f32,
    pub jump_speed: f32,
    pub jump_delay: f32,
}

#[derive(Component, Debug, Reflect, Inspectable, Copy, Clone, PartialEq, Eq)]
pub enum PlayerState {
    Controlled,
    Attacking,
    HitStun,
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let world_entity = WorldEntityBuilder::of_size(1.0).at_position(0.0, 10.0);
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    transform.scale = Vec3::new(1.0 / 3.2, 1.0 / 3.2, 1.0 / 3.2);
    commands
        .spawn_bundle(AnimatedSprite {
            sprite_animation: assets.load("sprites/Player.sprite"),
            transform,
            ..Default::default()
        })
        .insert_bundle(world_entity.rigid_body_bundle())
        .insert_bundle(world_entity.collider_bundle())
        .insert(ContactType::Player)
        .insert(Health::new(20))
        .insert(PlayerInputState::default())
        .insert(PlayerState::Controlled)
        .insert(PlayerStats {
            walk_speed: 10.0,
            air_speed: 4.0,
            jump_speed: 50.0,
            jump_delay: 0.15,
        })
        .insert(GroundedState::new(1.0, 1.0, 0.1))
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("player"));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_camera_system)
            .add_system(player_movement_system)
            .add_system(player_key_input_system)
            .add_system(player_hit_stun_recovery_system)
            .add_system(attack_impulse_update_system)
            .add_system(attack_brain_system)
            .add_system(attack_impulse_system)
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_player)
            .register_type::<PlayerStats>()
            .register_inspectable::<PlayerStats>()
            .register_type::<AttackImpulses>()
            .register_inspectable::<AttackImpulses>()
            .register_type::<PlayerAttackType>()
            .register_inspectable::<PlayerAttackType>()
            .register_type::<PlayerInputState>()
            .register_inspectable::<PlayerInputState>();
    }
}
