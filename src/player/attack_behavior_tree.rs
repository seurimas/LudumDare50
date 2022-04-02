use crate::{ai::powered::*, prelude::*, terrain::GroundedState};

#[derive(Component, Debug, Reflect, Inspectable, Clone)]
pub enum PlayerAttackType {
    Slash,
    AirSlash,
    Plunge,
}

impl PlayerAttackType {
    fn get_brain(&self) -> Box<dyn PoweredFunction<World = AttackImpulses> + Send + Sync> {
        match self {
            PlayerAttackType::Slash => PoweredTreeDef::Sequence(vec![
                PoweredTreeDef::User(AttackTreeNodeDef::Velocity(0.0, 0.0)),
                PoweredTreeDef::User(AttackTreeNodeDef::GoIntangible),
                PoweredTreeDef::User(AttackTreeNodeDef::SetFrame(1, 0.25)),
                PoweredTreeDef::User(AttackTreeNodeDef::WaitForFrame),
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

#[derive(Component, Debug, Reflect, Inspectable, Clone, Default)]
pub struct AttackImpulses {
    pub linvel: Option<Vec2>,
    pub animation_frame: Option<usize>,
    pub animation_time: f32,
    pub on_the_ground: bool,
    pub hit_minion: bool,
    pub intangible: bool,
}

impl AttackImpulses {
    pub fn reset(&mut self) {
        *self = Default::default()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AttackTreeNodeDef {
    Velocity(f32, f32),
    ClearVelocity,
    SetFrame(usize, f32),
    WaitForFrame,
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
            AttackTreeNodeDef::Velocity(x, y) => {
                attack.linvel = Some(Vec2::new(*x, *y));
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::ClearVelocity => {
                attack.linvel = None;
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::SetFrame(frame, time) => {
                attack.animation_frame = Some(*frame);
                attack.animation_time = *time;
                PoweredFunctionState::Complete(gas_left)
            }
            AttackTreeNodeDef::WaitForFrame => {
                if attack.animation_time <= 0.0 {
                    PoweredFunctionState::Complete(gas_left)
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
                if attack.hit_minion {
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

    fn reset(self: &mut Self, parameter: &mut Self::World) {}
}

impl UserNodeDefinition for AttackTreeNodeDef {
    type World = AttackImpulses;

    fn create_node(&self) -> Box<dyn PoweredFunction<World = Self::World> + Send + Sync> {
        Box::new(*self)
    }
}

pub fn attack_impulse_update_system(
    time: Res<Time>,
    mut attacker_query: Query<(&mut AttackImpulses, &GroundedState)>,
) {
    for (mut impulses, grounded) in attacker_query.iter_mut() {
        impulses.on_the_ground = grounded.on_the_ground();
        impulses.animation_time -= time.delta_seconds();
    }
}

#[derive(Component)]
pub struct AttackBrain(Box<dyn PoweredFunction<World = AttackImpulses> + Send + Sync>);

pub fn attack_brain_system(
    mut commands: Commands,
    uninitialized_query: Query<(Entity, &PlayerAttackType), Without<AttackBrain>>,
    mut attack_query: Query<(Entity, &mut AttackBrain, &mut AttackImpulses)>,
) {
    for (entity, mut attack_brain, mut player_attack) in attack_query.iter_mut() {
        match attack_brain.0.resume_with(99999, &mut player_attack) {
            PoweredFunctionState::Failed(_) | PoweredFunctionState::Complete(_) => {
                commands
                    .entity(entity)
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
            .insert(AttackBrain(attack.get_brain()))
            .insert(AttackImpulses::default());
    }
}

pub fn attack_impulse_system(
    mut attacker_query: Query<(
        &AttackImpulses,
        &mut TextureAtlasSprite,
        &mut RigidBodyVelocityComponent,
        &mut ContactType,
    )>,
) {
    for (impulses, mut sprite, mut velocity, mut contact_type) in attacker_query.iter_mut() {
        if impulses.intangible {
            *contact_type = ContactType::Inactive;
        }
        if let Some(frame) = impulses.animation_frame {
            sprite.index = frame;
        }
        if let Some(velocity_vec) = impulses.linvel {
            velocity.linvel = velocity_vec.into();
        }
    }
}
