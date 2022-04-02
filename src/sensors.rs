use crate::{
    animation::hitboxes::{Hitbox, SpriteSheetHitboxes},
    prelude::*,
};

#[derive(Debug)]
pub struct SensorEntity {
    pub center: Vec2,
    pub half_x: f32,
    pub half_y: f32,
}

impl SensorEntity {
    pub fn collider_bundle(&self) -> ColliderBundle {
        ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: ColliderShape::cuboid(self.half_x, self.half_y).into(),
            mass_properties: MassProperties::new(point![0.0, 0.0], 0.0, 0.0).into(),
            material: ColliderMaterial::new(0.0, 0.0).into(),
            flags: ActiveEvents::INTERSECTION_EVENTS.into(),
            ..Default::default()
        }
    }

    pub fn collider_parent(&self, handle: RigidBodyHandle) -> ColliderParentComponent {
        ColliderParent {
            handle,
            pos_wrt_parent: self.center.into(),
        }
        .into()
    }
}

pub fn sync_hitboxes(
    parent_query: Query<(
        Entity,
        Option<&Children>,
        &TextureAtlasSprite,
        &SpriteSheetHitboxes,
    )>,
    hitbox_query: Query<
        (Entity, &Hitbox),
        (
            With<ColliderPositionComponent>,
            With<ColliderShapeComponent>,
        ),
    >,
    configuration: Res<RapierConfiguration>,
    mut commands: Commands,
) {
    for (parent, children, texture_atlas_sprite, hitboxes) in parent_query.iter() {
        if let Some(hitboxes) = hitboxes.get_hitboxes(texture_atlas_sprite.index) {
            for current_hitbox in hitboxes {
                let mut flipped_hitbox = current_hitbox.clone();
                if texture_atlas_sprite.flip_x {
                    flipped_hitbox.flip_x();
                }
                if !has_child(&children, |child| {
                    if let Ok((_entity, hitbox)) = hitbox_query.get(**child) {
                        flipped_hitbox == *hitbox
                    } else {
                        false
                    }
                }) {
                    let sensor_entity = SensorEntity {
                        center: flipped_hitbox.get_center() / configuration.scale,
                        half_x: flipped_hitbox.get_half_x() / configuration.scale,
                        half_y: flipped_hitbox.get_half_y() / configuration.scale,
                    };
                    let child = commands
                        .spawn_bundle(sensor_entity.collider_bundle())
                        .insert(sensor_entity.collider_parent(parent.handle()))
                        .insert(ColliderDebugRender::default())
                        .insert(flipped_hitbox)
                        .insert(ColliderPositionSync::Discrete)
                        .id();
                    commands.entity(parent).add_child(child);
                }
            }
            if let Some(children) = children {
                children
                    .iter()
                    .filter_map(|child| hitbox_query.get(*child).ok())
                    .for_each(|(entity, hitbox)| {
                        if !hitboxes.contains(hitbox) {
                            commands.entity(entity).despawn();
                        }
                    });
            }
        }
    }
}
