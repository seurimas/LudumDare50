pub mod assets;
pub mod bundles;
pub mod component_types;
pub mod data_types;
pub mod hitboxes;
pub mod systems;
// LD50 note: Pulled in from a personal project.

use bevy::prelude::*;

use self::{
    assets::{sprite_animation_unpack_system, SpriteAnimationAsset, SpriteAnimationAssetLoader},
    component_types::AnimationState,
    systems::{animation_timer_system, direction_parameter_animation_system},
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_timer_system)
            .add_system(direction_parameter_animation_system)
            .register_type::<AnimationState>()
            .add_asset::<SpriteAnimationAsset>()
            .init_asset_loader::<SpriteAnimationAssetLoader>()
            .add_system(sprite_animation_unpack_system);
    }
}
