use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::geometry};

pub struct WorldEntityBuilder {
    pub radius: f32,
    pub velocity: Vec2,
    pub position: Vec2,
    pub mass: f32,
}

impl WorldEntityBuilder {
    pub fn of_size(radius: f32) -> Self {
        Self {
            radius,
            mass: radius * radius * 4.0,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
        }
    }

    pub fn with_velocity(self, x: f32, y: f32) -> Self {
        Self {
            velocity: Vec2::new(x, y),
            ..self
        }
    }

    pub fn at_position(self, x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..self
        }
    }

    pub fn collider_bundle(&self) -> ColliderBundle {
        ColliderBundle {
            shape: ColliderShapeComponent(geometry::ColliderShape::cuboid(
                self.radius,
                self.radius,
            )),
            mass_properties: MassProperties::new(point![0.0, 0.0], self.mass, 0.0).into(),
            material: ColliderMaterial::new(0.0, 0.0).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        }
    }

    pub fn rigid_body_bundle(&self) -> RigidBodyBundle {
        RigidBodyBundle {
            position: self.position.into(),
            velocity: RigidBodyVelocity {
                linvel: self.velocity.into(),
                angvel: 0.0,
            }
            .into(),
            ..Default::default()
        }
    }
}
