use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::PlayerStats;

pub fn player_camera_system(
    mut queries: QuerySet<(
        QueryState<(&PlayerStats, &GlobalTransform)>,
        QueryState<(&Camera, &mut GlobalTransform)>,
    )>,
) {
    let mut x = None;
    for (_player, player_transform) in queries.q0().iter() {
        x = Some(player_transform.translation.x);
    }
    if let Some(x) = x {
        for (_camera, mut camera_transform) in queries.q1().iter_mut() {
            camera_transform.translation.x = x;
        }
    }
}
