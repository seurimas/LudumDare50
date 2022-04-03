use crate::prelude::*;

#[derive(Component, Debug, Reflect, Inspectable, Default, Copy, Clone)]
pub struct TerrainBlock;

#[derive(Component, Debug, Reflect, Inspectable, Default, Copy, Clone)]
pub struct Parallax;

pub fn terrain_collider_bundle(width: f32, height: f32) -> ColliderBundle {
    ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::cuboid(width / 2.0, height / 2.0)),
        material: ColliderMaterial::new(1.0, 0.0).into(),
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
