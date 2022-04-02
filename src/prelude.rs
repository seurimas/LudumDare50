pub use crate::combat::*;
pub use bevy::prelude::*;
pub use bevy_inspector_egui::Inspectable;
pub use bevy_inspector_egui::RegisterInspectable;
pub use bevy_rapier2d::prelude::*;

pub fn get_vec_to_target(
    my_pos: &RigidBodyPositionComponent,
    target_pos: &RigidBodyPositionComponent,
) -> Vec2 {
    Vec2::new(
        target_pos.0.position.translation.x - my_pos.0.position.translation.x,
        target_pos.0.position.translation.y - my_pos.0.position.translation.y,
    )
}
