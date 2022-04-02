use crate::{
    minions::Minion,
    player::{AttackImpulses, PlayerState},
    prelude::*,
    terrain::GroundedState,
};
mod entity;
pub use entity::*;

const HIT_LAUNCH_SPEED: f32 = 20.0;

fn player_hit_stun(
    entity: Entity,
    launch_dir: Vec2,
    damage: i32,
    health_query: &mut Query<&mut Health>,
    velocity_query: &mut Query<&mut RigidBodyVelocityComponent>,
    player_state_query: &mut Query<(&mut GroundedState, &mut PlayerState)>,
) {
    if let Ok(mut health) = health_query.get_mut(entity) {
        health.current_health -= damage;
    }
    if let Ok(mut velocity) = velocity_query.get_mut(entity) {
        if launch_dir.x > 0.0 {
            velocity.linvel = Vec2::new(HIT_LAUNCH_SPEED, HIT_LAUNCH_SPEED).into();
        } else {
            velocity.linvel = Vec2::new(-HIT_LAUNCH_SPEED, HIT_LAUNCH_SPEED).into();
        }
    }
    if let Ok((mut grounded_state, mut player_state)) = player_state_query.get_mut(entity) {
        grounded_state.lift_off();
        *player_state = PlayerState::HitStun;
    }
}

fn get_position_hittables<'a>(
    entity_a: Entity,
    entity_b: Entity,
    hittable_query: &'a Query<&ContactType>,
    position_query: &'a Query<&RigidBodyPositionComponent>,
) -> Option<(
    &'a RigidBodyPositionComponent,
    &'a RigidBodyPositionComponent,
    &'a ContactType,
    &'a ContactType,
)> {
    if let (Ok(position_a), Ok(position_b)) =
        (position_query.get(entity_a), position_query.get(entity_b))
    {
        if let (Ok(hittable_a), Ok(hittable_b)) =
            (hittable_query.get(entity_a), hittable_query.get(entity_b))
        {
            return Some((position_a, position_b, hittable_a, hittable_b));
        }
    }
    None
}

fn player_hit_stun_system(
    mut contact_events: EventReader<ContactEvent>,
    position_query: Query<&RigidBodyPositionComponent>,
    hittable_query: Query<&ContactType>,
    mut health_query: Query<&mut Health>,
    mut player_state_query: Query<(&mut GroundedState, &mut PlayerState)>,
    mut velocity_query: Query<&mut RigidBodyVelocityComponent>,
) {
    for event in contact_events.iter() {
        match event {
            ContactEvent::Started(a, b) => {
                if let Some((position_a, position_b, hittable_a, hittable_b)) =
                    get_position_hittables(a.entity(), b.entity(), &hittable_query, &position_query)
                {
                    let hit_def = match (hittable_a, hittable_b) {
                        (ContactType::Player, ContactType::Minion(damage))
                        | (ContactType::Player, ContactType::MinionProjectile(damage)) => Some((
                            a.entity(),
                            get_vec_to_target(position_b, position_a),
                            damage,
                        )),
                        (ContactType::Minion(damage), ContactType::Player)
                        | (ContactType::MinionProjectile(damage), ContactType::Player) => Some((
                            b.entity(),
                            get_vec_to_target(position_a, position_b),
                            damage,
                        )),
                        _ => None,
                    };
                    if let Some((entity, launch_dir, damage)) = hit_def {
                        player_hit_stun(
                            entity,
                            launch_dir,
                            *damage,
                            &mut health_query,
                            &mut velocity_query,
                            &mut player_state_query,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn get_attacker(
    collider_entity: Entity,
    attacker_query: &Query<(Option<&Parent>, Option<&AttackImpulses>)>,
) -> Option<Entity> {
    if let Some((parent, m_attack)) = attacker_query.get(collider_entity).ok() {
        if m_attack.is_some() {
            Some(collider_entity)
        } else if let Some(parent) = parent {
            get_attacker(parent.0, attacker_query)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_attack_minion(
    entity_a: Entity,
    entity_b: Entity,
    attacker_query: &Query<(Option<&Parent>, Option<&AttackImpulses>)>,
    minion_query: &Query<(
        Entity,
        &mut Minion,
        &mut Health,
        &mut GroundedState,
        &mut RigidBodyVelocityComponent,
    )>,
) -> Option<(Entity, Entity)> {
    if let Some(attacker) = get_attacker(entity_a, attacker_query) {
        if let Ok(minion) = minion_query.get(entity_b) {
            return Some((attacker, minion.0));
        }
    } else if let Some(attacker) = get_attacker(entity_b, attacker_query) {
        if let Ok(minion) = minion_query.get(entity_a) {
            return Some((attacker, minion.0));
        }
    }
    None
}

fn attack_hit_stun_system(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut attacker_query: QuerySet<(
        QueryState<(Option<&Parent>, Option<&AttackImpulses>)>,
        QueryState<&mut AttackImpulses>,
    )>,
    mut minion_query: Query<(
        Entity,
        &mut Minion,
        &mut Health,
        &mut GroundedState,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    for event in intersection_events.iter() {
        if !event.intersecting {
            continue;
        }
        let entity_a = event.collider1.entity();
        let entity_b = event.collider2.entity();
        if let Some((attacker, minion)) =
            get_attack_minion(entity_a, entity_b, &attacker_query.q0(), &mut minion_query)
        {
            if let (
                Ok(mut impulses),
                Ok((entity, mut minion, mut health, mut grounded, mut velocity)),
            ) = (
                attacker_query.q1().get_mut(attacker),
                minion_query.get_mut(minion),
            ) {
                if minion.hit(impulses.attack_id) {
                    impulses.hit_minions.push(entity);
                    health.current_health -= impulses.attack_damage;
                    velocity.linvel = Vec2::new(0.0, HIT_LAUNCH_SPEED).into();
                    grounded.lift_off();
                }
            }
        }
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_hit_stun_system)
            .add_system(attack_hit_stun_system)
            .register_type::<Health>()
            .register_inspectable::<Health>()
            .register_type::<ContactType>()
            .register_inspectable::<ContactType>();
    }
}
