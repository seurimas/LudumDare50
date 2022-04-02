use crate::{ai::powered::*, prelude::*};

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
    pub timid: bool,
}

impl MinionThoughts {
    fn get_player_direction(&self) -> Option<Vec2> {
        self.player_at
            .map(|player| Vec2::new(player.x - self.self_at.x, player.y - self.self_at.y))
    }
}

pub enum MinionTreeNodeDef {
    OnTheGround,
    IsTimid,
    WaitForGround,
    PlayerVisible,
    PlayerInRange(f32, f32),
    LungeAtPlayer(f32, f32),
    LungeAway(f32, f32),
    ShootAtPlayer,
    Idle(f32),
    ResetOnHit(Box<PoweredTreeDef<MinionTreeNodeDef>>),
}

pub enum MinionTreeNode {
    OnTheGround,
    IsTimid,
    WaitForGround,
    PlayerVisible,
    PlayerInRange(f32, f32),
    LungeAtPlayer(f32, f32),
    LungeAway(f32, f32),
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
            MinionTreeNode::IsTimid => {
                if thoughts.timid {
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
            MinionTreeNode::LungeAway(speed, rise) => {
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
                    let away_dir = Vec2::new(-player_dir.x, 0.0);
                    thoughts.lunge_towards = Some((away_dir, *speed, *rise));
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
                    thoughts.idling = true;
                    return PoweredFunctionState::Waiting(gas_left);
                } else {
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
            MinionTreeNodeDef::IsTimid => Box::new(MinionTreeNode::IsTimid),
            MinionTreeNodeDef::WaitForGround => Box::new(MinionTreeNode::WaitForGround),
            MinionTreeNodeDef::PlayerVisible => Box::new(MinionTreeNode::PlayerVisible),
            MinionTreeNodeDef::PlayerInRange(h_w, h_h) => {
                Box::new(MinionTreeNode::PlayerInRange(*h_w, *h_h))
            }
            MinionTreeNodeDef::LungeAtPlayer(speed, rise) => {
                Box::new(MinionTreeNode::LungeAtPlayer(*speed, *rise))
            }
            MinionTreeNodeDef::LungeAway(speed, rise) => {
                Box::new(MinionTreeNode::LungeAway(*speed, *rise))
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
