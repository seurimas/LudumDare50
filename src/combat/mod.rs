use crate::{player::PlayerState, prelude::*, terrain::GroundedState};
mod entity;
pub use entity::*;

const HIT_LAUNCH_SPEED: f32 = 20.0;

fn hit_stun_system(
    mut contact_events: EventReader<ContactEvent>,
    mut health_query: Query<&mut Health>,
    hittable_query: Query<&ContactType>,
    mut player_state_query: Query<(&mut GroundedState, &mut PlayerState)>,
    position_query: Query<&RigidBodyPositionComponent>,
    mut velocity_query: Query<&mut RigidBodyVelocityComponent>,
) {
    for event in contact_events.iter() {
        match event {
            ContactEvent::Started(a, b) => {
                if let (Ok(position_a), Ok(position_b)) = (
                    position_query.get(a.entity()),
                    position_query.get(b.entity()),
                ) {
                    let hit_def = if let (Ok(hittable_a), Ok(hittable_b)) = (
                        hittable_query.get(a.entity()),
                        hittable_query.get(b.entity()),
                    ) {
                        match (hittable_a, hittable_b) {
                            (ContactType::Player, ContactType::Minion(damage))
                            | (ContactType::Player, ContactType::MinionProjectile(damage)) => {
                                Some((
                                    a.entity(),
                                    get_vec_to_target(position_b, position_a),
                                    damage,
                                ))
                            }
                            (ContactType::Minion(damage), ContactType::Player)
                            | (ContactType::MinionProjectile(damage), ContactType::Player) => {
                                Some((
                                    b.entity(),
                                    get_vec_to_target(position_a, position_b),
                                    damage,
                                ))
                            }
                            _ => None,
                        }
                    } else {
                        None
                    };
                    if let Some((entity, launch_dir, damage)) = hit_def {
                        if let Ok(mut health) = health_query.get_mut(entity) {
                            health.current_health -= damage;
                        }
                        if let Ok(mut velocity) = velocity_query.get_mut(entity) {
                            if launch_dir.x > 0.0 {
                                velocity.linvel =
                                    Vec2::new(HIT_LAUNCH_SPEED, HIT_LAUNCH_SPEED).into();
                            } else {
                                velocity.linvel =
                                    Vec2::new(-HIT_LAUNCH_SPEED, HIT_LAUNCH_SPEED).into();
                            }
                        }
                        if let Ok((mut grounded_state, mut player_state)) =
                            player_state_query.get_mut(entity)
                        {
                            grounded_state.lift_off();
                            *player_state = PlayerState::HitStun;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hit_stun_system)
            .register_type::<Health>()
            .register_inspectable::<Health>()
            .register_type::<ContactType>()
            .register_inspectable::<ContactType>();
    }
}
