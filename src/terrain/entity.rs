use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::{prelude::*, rapier::geometry::ColliderShape};

#[derive(Component, Debug, Reflect, Inspectable, Default, Copy, Clone)]
pub struct TerrainBlock;

pub fn terrain_collider_bundle(width: f32, height: f32) -> ColliderBundle {
    ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::cuboid(width / 2.0, height / 2.0)),
        material: ColliderMaterial::new(0.0, 0.0).into(),
        flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
        ..Default::default()
    }
}

pub fn terrain_rigid_body_bundle(position: Vec2) -> RigidBodyBundle {
    RigidBodyBundle {
        body_type: RigidBodyType::Static.into(),
        position: position.into(),
        ..Default::default()
    }
}

pub fn spawn_terrain(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn_bundle(terrain_collider_bundle(width, height))
        .insert_bundle(terrain_rigid_body_bundle(position))
        .insert(TerrainBlock)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
}
