use crate::prelude::*;

pub struct WorldEntityBuilder {
    pub radius: f32,
    pub velocity: Vec2,
    pub position: Vec2,
    pub mass: f32,
    pub collision_group: u32,
    pub collision_filter: u32,
    pub solver_group: u32,
    pub solver_filter: u32,
}

impl WorldEntityBuilder {
    pub fn of_size(radius: f32) -> Self {
        Self {
            radius,
            mass: radius * radius * 4.0,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            collision_group: 0xffff,
            collision_filter: 0xffff,
            solver_group: 0xffff,
            solver_filter: 0xffff,
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

    pub fn with_collision_group(self, collision_group: u32) -> Self {
        Self {
            collision_group,
            ..self
        }
    }

    pub fn with_collision_filter(self, collision_filter: u32) -> Self {
        Self {
            collision_filter,
            ..self
        }
    }

    pub fn with_solver_group(self, solver_group: u32) -> Self {
        Self {
            solver_group,
            ..self
        }
    }

    pub fn with_solver_filter(self, solver_filter: u32) -> Self {
        Self {
            solver_filter,
            ..self
        }
    }

    pub fn collider_bundle(&self) -> ColliderBundle {
        ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(self.radius, self.radius)),
            mass_properties: MassProperties::new(point![0.0, 0.0], self.mass, 0.0).into(),
            material: ColliderMaterial::new(1.0, 0.0).into(),
            flags: ColliderFlags {
                active_events: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS),
                solver_groups: InteractionGroups::new(self.solver_group, self.solver_filter),
                collision_groups: InteractionGroups::new(
                    self.collision_group,
                    self.collision_filter,
                ),
                ..Default::default()
            }
            .into(),
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
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            ..Default::default()
        }
    }
}
