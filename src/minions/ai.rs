use crate::prelude::*;

use crate::{
    ai::powered::{PoweredFunction, PoweredTreeDef},
    animation::component_types::{AnimationState, ParameterizedSpriteAnimationSet},
    player::PlayerStats,
    terrain::GroundedState,
};

use super::behavior_tree::{MinionThoughts, MinionTreeNodeDef};
use super::{Minion, MinionBrain};

pub fn minion_brain() -> Box<dyn PoweredFunction<World = MinionThoughts> + Send + Sync> {
    let brain_def = PoweredTreeDef::User(MinionTreeNodeDef::ResetOnHit(Box::new(
        PoweredTreeDef::Selector(vec![
            PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(MinionTreeNodeDef::OnTheGround),
                PoweredTreeDef::User(MinionTreeNodeDef::PlayerVisible),
                PoweredTreeDef::User(MinionTreeNodeDef::IsTimid),
                PoweredTreeDef::User(MinionTreeNodeDef::LungeAway(20.0, 10.0)),
                PoweredTreeDef::User(MinionTreeNodeDef::WaitForGround),
                PoweredTreeDef::User(MinionTreeNodeDef::Idle(0.25)),
            ]),
            PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(MinionTreeNodeDef::OnTheGround),
                PoweredTreeDef::User(MinionTreeNodeDef::PlayerVisible),
                PoweredTreeDef::User(MinionTreeNodeDef::PlayerInRange(5.0, 1.0)),
                PoweredTreeDef::User(MinionTreeNodeDef::LungeAtPlayer(20.0, 10.0)),
                PoweredTreeDef::User(MinionTreeNodeDef::WaitForGround),
                PoweredTreeDef::User(MinionTreeNodeDef::Idle(1.0)),
            ]),
            PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(MinionTreeNodeDef::OnTheGround),
                PoweredTreeDef::User(MinionTreeNodeDef::PlayerVisible),
                PoweredTreeDef::User(MinionTreeNodeDef::Idle(1.0)),
                PoweredTreeDef::User(MinionTreeNodeDef::LungeAtPlayer(20.0, 10.0)),
                PoweredTreeDef::User(MinionTreeNodeDef::WaitForGround),
                PoweredTreeDef::User(MinionTreeNodeDef::Idle(1.0)),
            ]),
            PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(MinionTreeNodeDef::WaitForGround),
                PoweredTreeDef::User(MinionTreeNodeDef::Idle(1.0)),
            ]),
        ]),
    )));
    brain_def.create_tree()
}

pub fn minion_thought_update_system(
    time: Res<Time>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut minion_query: Query<(
        Entity,
        &Minion,
        &mut MinionThoughts,
        &Health,
        &GroundedState,
        &ParameterizedSpriteAnimationSet,
        &AnimationState,
    )>,
    player_query: Query<&PlayerStats>,
    position_query: Query<&RigidBodyPositionComponent>,
) {
    for (entity, minion, mut thoughts, health, grounded, animation_set, animation_state) in
        minion_query.iter_mut()
    {
        thoughts.frame_time = time.delta_seconds();
        thoughts.player_at = None;
        thoughts.idling = false;
        thoughts.on_the_ground = grounded.on_the_ground();
        thoughts.animation = animation_state.get_animation().clone();
        thoughts.animation_complete = animation_set.animation_complete(animation_state);
        thoughts.timid = health.current_health <= minion.timidity;
        if let Ok(minion_pos) = position_query.get(entity) {
            thoughts.self_at = Vec2::new(
                minion_pos.0.position.translation.x,
                minion_pos.0.position.translation.y,
            );
            let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
            let shape = Ball::new(minion.los_distance);
            let shape_pos = minion_pos.0.position.translation.into();
            let groups = InteractionGroups::all();
            let filter = None;
            query_pipeline.intersections_with_shape(
                &collider_set,
                &shape_pos,
                &shape,
                groups,
                filter,
                |handle| {
                    if player_query.get(handle.entity()).is_ok() {
                        if let Ok(player_pos) = position_query.get(handle.entity()) {
                            thoughts.player_at = Some(Vec2::new(
                                player_pos.0.position.translation.x,
                                player_pos.0.position.translation.y,
                            ));
                        }
                        false
                    } else {
                        true
                    }
                },
            );
        }
    }
}

pub fn minion_brain_system(
    mut minion_query: Query<(Entity, &mut MinionBrain, &mut MinionThoughts)>,
) {
    for (entity, mut minion_brain, mut minion_thoughts) in minion_query.iter_mut() {
        minion_brain.0.resume_with(99999, &mut minion_thoughts);
    }
}

pub fn minion_impulse_system(
    mut minion_query: Query<(
        Entity,
        &MinionThoughts,
        &mut TextureAtlasSprite,
        &mut AnimationState,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
) {
    for (entity, thoughts, mut sprite, mut animation_state, mut velocity, mass) in
        minion_query.iter_mut()
    {
        if thoughts.idling {
            animation_state.transition_to("Idle", true);
        } else if let Some((lunge_dir, lunge_speed, lunge_rise)) = thoughts.lunge_towards {
            if animation_state.try_transition_to("Lunge", false) {
                let normalized = lunge_dir.normalize_or_zero();
                velocity.apply_impulse(
                    mass,
                    Vec2::new(
                        normalized.x * lunge_speed,
                        normalized.y * lunge_speed + lunge_rise,
                    )
                    .into(),
                );
                sprite.flip_x = lunge_dir.x < 0.0;
            }
        }
    }
}
