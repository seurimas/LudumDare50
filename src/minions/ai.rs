use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    ai::powered::{PoweredFunction, PoweredFunctionState, PoweredTreeDef, UserNodeDefinition},
    animation::component_types::{AnimationState, ParameterizedSpriteAnimationSet},
    player::PlayerStats,
    terrain::GroundedState,
};

use super::{Minion, MinionBrain};

#[derive(Component, Debug, Reflect, Inspectable, Default, Clone)]
pub struct MinionThoughts {
    pub self_at: Vec2,
    pub player_at: Option<Vec2>,
    pub lunge_towards: Option<(Vec2, f32, f32)>,
    pub shoot_at: Option<Vec2>,
    pub hit_stun: bool,
    pub frame_time: f32,
    pub idling: bool,
    pub on_the_ground: bool,
    pub animation: String,
    pub animation_complete: bool,
}

impl MinionThoughts {
    fn get_player_direction(&self) -> Option<Vec2> {
        self.player_at
            .map(|player| Vec2::new(player.x - self.self_at.x, player.y - self.self_at.y))
    }
}

pub enum MinionTreeNodeDef {
    OnTheGround,
    WaitForGround,
    PlayerVisible,
    PlayerInRange(f32, f32),
    LungeAtPlayer(f32, f32),
    ShootAtPlayer,
    Idle(f32),
    ResetOnHit(Box<PoweredTreeDef<MinionTreeNodeDef>>),
}

pub enum MinionTreeNode {
    OnTheGround,
    WaitForGround,
    PlayerVisible,
    PlayerInRange(f32, f32),
    LungeAtPlayer(f32, f32),
    ShootAtPlayer,
    Idle { duration: f32, progress: f32 },
    ResetOnHit(Box<dyn PoweredFunction<World = MinionThoughts> + Send + Sync>),
}

impl PoweredFunction for MinionTreeNode {
    type World = MinionThoughts;

    fn resume_with(
        self: &mut Self,
        gas_left: i32,
        thoughts: &mut Self::World,
    ) -> PoweredFunctionState {
        match self {
            MinionTreeNode::OnTheGround => {
                if thoughts.on_the_ground {
                    return PoweredFunctionState::Complete(gas_left);
                } else {
                    return PoweredFunctionState::Failed(gas_left);
                }
            }
            MinionTreeNode::WaitForGround => {
                if thoughts.on_the_ground {
                    return PoweredFunctionState::Complete(gas_left);
                } else {
                    return PoweredFunctionState::Waiting(gas_left);
                }
            }
            MinionTreeNode::PlayerVisible => {
                if thoughts.player_at.is_some() {
                    return PoweredFunctionState::Complete(gas_left);
                } else {
                    return PoweredFunctionState::Failed(gas_left);
                }
            }
            MinionTreeNode::PlayerInRange(x_diff, y_diff) => {
                if let Some(player_dir) = thoughts.get_player_direction() {
                    if player_dir.x.abs() > *x_diff {
                        return PoweredFunctionState::Failed(gas_left);
                    } else if player_dir.y.abs() > *y_diff {
                        return PoweredFunctionState::Failed(gas_left);
                    } else {
                        return PoweredFunctionState::Complete(gas_left);
                    }
                }
                return PoweredFunctionState::Failed(gas_left);
            }
            MinionTreeNode::LungeAtPlayer(speed, rise) => {
                if thoughts.animation.eq("Lunge") {
                    if thoughts.animation_complete {
                        thoughts.lunge_towards = None;
                        return PoweredFunctionState::Complete(gas_left);
                    } else {
                        return PoweredFunctionState::Waiting(gas_left);
                    }
                } else if !thoughts.on_the_ground {
                    thoughts.lunge_towards = None;
                    return PoweredFunctionState::Failed(gas_left);
                } else if let Some(player_dir) = thoughts.get_player_direction() {
                    thoughts.lunge_towards = Some((player_dir, *speed, *rise));
                    return PoweredFunctionState::Waiting(gas_left);
                }
                thoughts.lunge_towards = None;
                return PoweredFunctionState::Failed(gas_left);
            }
            MinionTreeNode::ShootAtPlayer => {
                if let Some(player_dir) = thoughts.get_player_direction() {
                    thoughts.shoot_at = Some(player_dir);
                    return PoweredFunctionState::Complete(gas_left);
                }
                return PoweredFunctionState::Failed(gas_left);
            }
            MinionTreeNode::Idle { duration, progress } => {
                *progress += thoughts.frame_time;
                if progress < duration {
                    println!("Idling waiting");
                    thoughts.idling = true;
                    return PoweredFunctionState::Waiting(gas_left);
                } else {
                    println!("Idle done");
                    self.reset(thoughts);
                    return PoweredFunctionState::Complete(gas_left);
                }
            }
            MinionTreeNode::ResetOnHit(child) => {
                if thoughts.hit_stun {
                    child.reset(thoughts);
                    return PoweredFunctionState::Complete(gas_left);
                } else {
                    return child.resume_with(gas_left, thoughts);
                }
            }
        }
    }

    fn reset(self: &mut Self, parameter: &mut Self::World) {
        match self {
            MinionTreeNode::Idle { progress, .. } => {
                *progress = 0.0;
            }
            MinionTreeNode::ResetOnHit(child) => child.reset(parameter),
            _ => {}
        }
    }
}

impl UserNodeDefinition for MinionTreeNodeDef {
    type World = MinionThoughts;

    fn create_node(&self) -> Box<dyn PoweredFunction<World = Self::World> + Send + Sync> {
        match self {
            MinionTreeNodeDef::OnTheGround => Box::new(MinionTreeNode::OnTheGround),
            MinionTreeNodeDef::WaitForGround => Box::new(MinionTreeNode::WaitForGround),
            MinionTreeNodeDef::PlayerVisible => Box::new(MinionTreeNode::PlayerVisible),
            MinionTreeNodeDef::PlayerInRange(h_w, h_h) => {
                Box::new(MinionTreeNode::PlayerInRange(*h_w, *h_h))
            }
            MinionTreeNodeDef::LungeAtPlayer(speed, rise) => {
                Box::new(MinionTreeNode::LungeAtPlayer(*speed, *rise))
            }
            MinionTreeNodeDef::ShootAtPlayer => Box::new(MinionTreeNode::ShootAtPlayer),
            MinionTreeNodeDef::Idle(duration) => Box::new(MinionTreeNode::Idle {
                duration: *duration,
                progress: 0.0,
            }),
            MinionTreeNodeDef::ResetOnHit(child) => {
                Box::new(MinionTreeNode::ResetOnHit(child.create_tree()))
            }
        }
    }
}

pub fn minion_brain() -> Box<dyn PoweredFunction<World = MinionThoughts> + Send + Sync> {
    let brain_def = PoweredTreeDef::User(MinionTreeNodeDef::ResetOnHit(Box::new(
        PoweredTreeDef::Selector(vec![
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
        &GroundedState,
        &ParameterizedSpriteAnimationSet,
        &AnimationState,
    )>,
    player_query: Query<&PlayerStats>,
    position_query: Query<&RigidBodyPositionComponent>,
) {
    for (entity, minion, mut thoughts, grounded, animation_set, animation_state) in
        minion_query.iter_mut()
    {
        thoughts.frame_time = time.delta_seconds();
        thoughts.player_at = None;
        thoughts.idling = false;
        thoughts.on_the_ground = grounded.on_the_ground();
        thoughts.animation = animation_state.get_animation().clone();
        thoughts.animation_complete = animation_set.animation_complete(animation_state);
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
            println!("Idling");
            animation_state.transition_to("Idle", true);
        } else if let Some((lunge_dir, lunge_speed, lunge_rise)) = thoughts.lunge_towards {
            if animation_state.try_transition_to("Lunge", false) {
                println!("LUNGING");
                if lunge_dir.length_squared() > 0.0 {
                    let normalized = lunge_dir.normalize();
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
}
