use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::physics::RigidBodyPositionSync;

mod ai;
use self::ai::*;

use crate::{
    ai::powered::PoweredFunction, animation::bundles::AnimatedSprite,
    base_bundles::WorldEntityBuilder, terrain::GroundedState,
};

#[derive(Component, Debug, Reflect, Inspectable, Default, Copy, Clone)]
pub struct Minion {
    los_distance: f32,
    sees_player: bool,
    is_attacking: bool,
    waiting_for: f32,
}

#[derive(Component)]
pub struct MinionBrain(Box<dyn PoweredFunction<World = MinionThoughts> + Send + Sync>);

impl Minion {
    pub fn new(los_distance: f32) -> Self {
        Minion {
            los_distance,
            sees_player: false,
            is_attacking: false,
            waiting_for: 0.0,
        }
    }
}

fn spawn_minion(mut commands: Commands, assets: Res<AssetServer>) {
    let world_entity = WorldEntityBuilder::of_size(0.5).at_position(5.0, 10.0);
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    transform.scale = Vec3::new(1.0 / 6.4, 1.0 / 6.4, 1.0 / 6.4);
    commands
        .spawn_bundle(AnimatedSprite {
            sprite_animation: assets.load("sprites/Minion.sprite"),
            transform,
            ..Default::default()
        })
        .insert_bundle(world_entity.rigid_body_bundle())
        .insert_bundle(world_entity.collider_bundle())
        .insert(Minion::new(10.0))
        .insert(MinionThoughts::default())
        .insert(MinionBrain(minion_brain()))
        .insert(GroundedState::new(0.5, 0.5, 0.1))
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("player"));
}

pub struct MinionsPlugin;

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_minion)
            .add_system(minion_thought_update_system)
            .add_system(minion_brain_system)
            .add_system(minion_impulse_system)
            .register_type::<Minion>()
            .register_type::<MinionThoughts>()
            .register_inspectable::<MinionThoughts>()
            .register_inspectable::<Minion>();
    }
}
