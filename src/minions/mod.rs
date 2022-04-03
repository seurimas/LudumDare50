use crate::{combat::ContactType, prelude::*};

mod ai;
pub mod behavior_tree;
use self::{ai::*, behavior_tree::MinionThoughts};

use crate::{
    ai::powered::PoweredFunction, animation::bundles::AnimatedSprite,
    base_bundles::WorldEntityBuilder, terrain::GroundedState,
};

#[derive(Component, Debug, Reflect, Inspectable, Default, Clone)]
pub struct Minion {
    los_distance: f32,
    timidity: i32,
    hit_by: Vec<u32>,
}

#[derive(Component)]
pub struct MinionBrain(Box<dyn PoweredFunction<World = MinionThoughts> + Send + Sync>);

impl Minion {
    pub fn new(los_distance: f32, timidity: i32) -> Self {
        Minion {
            los_distance,
            timidity,
            hit_by: Vec::new(),
        }
    }

    pub fn hit(&mut self, attack_id: u32) -> bool {
        if self.hit_by.contains(&attack_id) {
            false
        } else {
            self.hit_by.push(attack_id);
            true
        }
    }
}

fn spawn_minion(mut commands: Commands, assets: Res<AssetServer>) {
    let world_entity = WorldEntityBuilder::of_size(0.5)
        .at_position(5.0, 10.0)
        .with_solver_group(0b0010)
        .with_solver_filter(0b0010);
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    commands
        .spawn_bundle(AnimatedSprite {
            sprite_animation: assets.load("sprites/Minion.sprite"),
            transform,
            ..Default::default()
        })
        .insert_bundle(world_entity.rigid_body_bundle())
        .insert_bundle(world_entity.collider_bundle())
        .insert(ContactType::Minion(1))
        .insert(Health::new(3))
        .insert(Minion::new(10.0, 1))
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
