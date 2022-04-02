use crate::{
    animation::hitboxes::{Hitbox, SpriteSheetHitboxes},
    prelude::*,
    sensors::SensorEntity,
    terrain::GroundedState,
};

use super::PlayerState;

pub fn player_hit_stun_recovery_system(
    mut player_query: Query<(&GroundedState, &mut PlayerState)>,
) {
    for (grounded_state, mut player_state) in player_query.iter_mut() {
        if *player_state == PlayerState::HitStun && grounded_state.on_the_ground() {
            *player_state = PlayerState::Controlled;
        }
    }
}
