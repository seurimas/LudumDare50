use super::{assets::SpriteAnimationAsset, component_types::*};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct AnimatedSprite {
    pub sprite: TextureAtlasSprite,
    pub sprite_animation: Handle<SpriteAnimationAsset>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub animation_state: AnimationState,
}
