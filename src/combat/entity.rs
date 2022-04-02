use crate::prelude::*;

#[derive(Component, Debug, Reflect, Inspectable, Default, Clone)]
pub struct Health {
    pub max_health: i32,
    pub current_health: i32,
    pub hit_stun: f32,
    pub invulnerability: f32,
}

impl Health {
    pub fn new(max_health: i32) -> Self {
        Health {
            max_health,
            current_health: max_health,
            hit_stun: 0.0,
            invulnerability: 0.0,
        }
    }
}

#[derive(Component, Debug, Reflect, Inspectable, Clone)]
pub enum ContactType {
    Player,
    Minion(i32),
    MinionProjectile(i32),
    Inactive,
}

impl Default for ContactType {
    fn default() -> Self {
        ContactType::Inactive
    }
}
