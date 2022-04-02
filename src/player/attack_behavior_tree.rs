use crate::{
    ai::powered::*,
    animation::component_types::{AnimationState, ParameterizedSpriteAnimationSet},
    prelude::*,
    terrain::GroundedState,
};

use super::PlayerState;

#[derive(Component, Debug, Reflect, Inspectable, Clone)]
pub enum PlayerAttackType {
    Slash,
    RunningSlash(f32),
    AirSlash,
    Plunge,
}

impl PlayerAttackType {
    fn get_brain(&self) -> Box<dyn PoweredFunction<World = AttackImpulses> + Send + Sync> {
        println!("{:?}", self);
        match self {
            PlayerAttackType::Slash => PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(AttackTreeNodeDef::Velocity(0.0, 0.0)),
                PoweredTreeDef::User(AttackTreeNodeDef::GoIntangible),
                PoweredTreeDef::User(AttackTreeNodeDef::SetDamage(1)),
                PoweredTreeDef::User(AttackTreeNodeDef::PlayAnimation("Slash".to_string())),
                PoweredTreeDef::User(AttackTreeNodeDef::WaitForAnimation("Slash".to_string())),
            ]),
            PlayerAttackType::RunningSlash(x) => PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(AttackTreeNodeDef::Velocity(*x, 0.0)),
                PoweredTreeDef::User(AttackTreeNodeDef::GoIntangible),
                PoweredTreeDef::User(AttackTreeNodeDef::SetDamage(1)),
                PoweredTreeDef::User(AttackTreeNodeDef::PlayAnimation("RunningSlash".to_string())),
                PoweredTreeDef::User(AttackTreeNodeDef::WaitForAnimation(
                    "RunningSlash".to_string(),
                )),
            ]),
            PlayerAttackType::AirSlash => todo!(),
            PlayerAttackType::Plunge => todo!(),
        }
        .create_tree()
    }
}

impl Default for PlayerAttackType {
    fn default() -> Self {
        PlayerAttackType::Slash
    }
}

#[derive(Component, Debug, Reflect, Clone, Default)]
pub struct AttackImpulses {
    pub attack_id: u32,
    pub attack_damage: i32,
    pub speed: Vec2,
    pub set_speed: Option<Vec2>,
    pub play_animation: Option<String>,
    pub animation_frame: Option<usize>,
    pub animation_time: f32,
    pub on_the_ground: bool,
    pub animation: String,
    pub animation_complete: bool,
    pub hit_minions: Vec<Entity>,
    pub intangible: bool,
}

impl AttackImpulses {
    pub fn new(attack_id: u32) -> Self {
        AttackImpulses {
            attack_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttackTreeNodeDef {
    SetDamage(i32),
    Velocity(f32, f32),
    ClearVelocity,
    PlayAnimation(String),
    SetFrame(usize, f32),
    WaitForFrame,
    WaitForAnimation(String),
    WaitForGround,
    OnTheGround,
    WaitForHit,
    GoIntangible,
}

impl PoweredFunction for AttackTreeNodeDef {
    type World = AttackImpulses;

    fn resume_with(
        self: &mut Self,
        gas_left: i32,
        attack: &mut Self::World,
    ) -> PoweredFunctionState {
        match self {
            AttackTreeNodeDef::SetDamage(damage) => {
                attack.attack_damage = *damage;
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::Velocity(x, y) => {
                attack.set_speed = Some(Vec2::new(*x, *y));
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::ClearVelocity => {
                attack.set_speed = None;
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::SetFrame(frame, time) => {
                attack.animation_frame = Some(*frame);
                attack.animation_time = *time;
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::PlayAnimation(animation) => {
                attack.play_animation = Some(animation.clone());
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::WaitForFrame => {
                if attack.animation_time <= 0.0 {
                    PoweredFunctionState::Complete(gas_left)
                } else {
                    PoweredFunctionState::Waiting(gas_left)
                }
            }
            AttackTreeNodeDef::WaitForAnimation(animation) => {
                if attack.animation.eq(animation) {
                    attack.play_animation = None;
                    if attack.animation_complete {
                        PoweredFunctionState::Complete(gas_left)
                    } else {
                        PoweredFunctionState::Waiting(gas_left)
                    }
                } else if attack.play_animation.is_none() {
                    PoweredFunctionState::Failed(gas_left)
                } else {
                    PoweredFunctionState::Waiting(gas_left)
                }
            }
            AttackTreeNodeDef::WaitForGround => {
                if attack.on_the_ground {
                    PoweredFunctionState::Complete(gas_left)
                } else {
                    PoweredFunctionState::Waiting(gas_left)
                }
            }
            AttackTreeNodeDef::OnTheGround => {
                if attack.on_the_ground {
                    PoweredFunctionState::Complete(gas_left)
                } else {
                    PoweredFunctionState::Failed(gas_left)
                }
            }
            AttackTreeNodeDef::WaitForHit => {
                if !attack.hit_minions.is_empty() {
                    PoweredFunctionState::Complete(gas_left)
                } else {
                    PoweredFunctionState::Waiting(gas_left)
                }
            }
            AttackTreeNodeDef::GoIntangible => {
                attack.intangible = true;
                PoweredFunctionState::Complete(gas_left)
            }
        }
    }

    fn reset(self: &mut Self, parameter: &mut Self::World) {
        // Stateless, nothing to reset for us.
    }
}

impl UserNodeDefinition for AttackTreeNodeDef {
    type World = AttackImpulses;

    fn create_node(&self) -> Box<dyn PoweredFunction<World = Self::World> + Send + Sync> {
        Box::new(self.clone())
    }
}

pub fn attack_impulse_update_system(
    time: Res<Time>,
    mut attacker_query: Query<(
        &mut AttackImpulses,
        &GroundedState,
        &ParameterizedSpriteAnimationSet,
        &AnimationState,
        &RigidBodyVelocityComponent,
    )>,
) {
    for (mut impulses, grounded, animation_set, animation_state, velocity) in
        attacker_query.iter_mut()
    {
        impulses.on_the_ground = grounded.on_the_ground();
        impulses.animation_time -= time.delta_seconds();
        impulses.animation = animation_state.get_animation().to_string();
        impulses.animation_complete = animation_set.animation_complete(animation_state);
        impulses.speed = velocity.linvel.into();
    }
}

#[derive(Component)]
pub struct AttackBrain(Box<dyn PoweredFunction<World = AttackImpulses> + Send + Sync>);

pub fn attack_brain_system(
    mut commands: Commands,
    mut attack_id: Local<u32>,
    uninitialized_query: Query<(Entity, &PlayerAttackType), Without<AttackBrain>>,
    mut attack_query: Query<(Entity, &mut AttackBrain, &mut AttackImpulses)>,
) {
    for (entity, mut attack_brain, mut player_attack) in attack_query.iter_mut() {
        match attack_brain.0.resume_with(99999, &mut player_attack) {
            PoweredFunctionState::Failed(_) | PoweredFunctionState::Complete(_) => {
                commands
                    .entity(entity)
                    .insert(PlayerState::Controlled)
                    .remove::<PlayerAttackType>()
                    .remove::<AttackBrain>()
                    .remove::<AttackImpulses>();
            }
            _ => {}
        }
    }
    for (entity, attack) in uninitialized_query.iter() {
        commands
            .entity(entity)
            .insert(PlayerState::Attacking)
            .insert(AttackBrain(attack.get_brain()))
            .insert(AttackImpulses::new(*attack_id));
        *attack_id += 1;
    }
}

pub fn attack_impulse_system(
    mut attacker_query: Query<(
        &AttackImpulses,
        &mut TextureAtlasSprite,
        &mut AnimationState,
        &mut RigidBodyVelocityComponent,
        &mut ContactType,
    )>,
) {
    for (impulses, mut sprite, mut animation_state, mut velocity, mut contact_type) in
        attacker_query.iter_mut()
    {
        if impulses.intangible {
            *contact_type = ContactType::Inactive;
        }
        if let Some(frame) = impulses.animation_frame {
            sprite.index = frame;
        }
        if let Some(velocity_vec) = impulses.set_speed {
            velocity.linvel = velocity_vec.into();
        }
        if let Some(animation) = &impulses.play_animation {
            animation_state.transition_to(animation, false);
        }
    }
}
