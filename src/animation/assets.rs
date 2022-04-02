use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture::{FileTextureError, ImageType},
};
use serde::{Deserialize, Serialize};

use super::{component_types::ParameterizedSpriteAnimationSet, hitboxes::SpriteSheetHitboxes};

#[derive(Default, Debug, TypeUuid)]
#[uuid = "d5ecf4c3-6d03-49db-a6ad-1e4cd5c2369c"]
pub struct SpriteAnimationAsset {
    pub texture_atlas: Handle<TextureAtlas>,
    pub parameterized_animation_set: ParameterizedSpriteAnimationSet,
    pub hitboxes: Option<SpriteSheetHitboxes>,
}

#[derive(Debug, Serialize, Deserialize, TypeUuid)]
#[uuid = "33599347-16e6-4170-be40-a12e5a64253a"]
pub struct SpriteAnimationAssetDef {
    pub atlas_tile_size: Vec2,
    pub atlas_columns: usize,
    pub atlas_rows: usize,
    pub atlas_path: String,
    pub animation_path: String,
    pub hitboxes: Option<SpriteSheetHitboxes>,
}

#[derive(Default)]
pub struct SpriteAnimationAssetLoader;

impl AssetLoader for SpriteAnimationAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset_def = ron::de::from_bytes::<SpriteAnimationAssetDef>(bytes)?;
            let texture_bytes = load_context.read_asset_bytes(asset_def.atlas_path).await?;
            let image = Image::from_buffer(&texture_bytes, ImageType::Extension("png"))?;
            let texture = load_context.set_labeled_asset("texture", LoadedAsset::new(image));
            let texture_atlas_real = TextureAtlas::from_grid(
                texture,
                asset_def.atlas_tile_size,
                asset_def.atlas_columns,
                asset_def.atlas_rows,
            );
            let texture_atlas =
                load_context.set_labeled_asset("atlas", LoadedAsset::new(texture_atlas_real));
            let animation_bytes = load_context
                .read_asset_bytes(asset_def.animation_path.clone())
                .await?;
            let parameterized_animation_set =
                ron::de::from_bytes::<ParameterizedSpriteAnimationSet>(&animation_bytes)?;
            load_context.set_default_asset(LoadedAsset::new(SpriteAnimationAsset {
                texture_atlas,
                parameterized_animation_set,
                hitboxes: asset_def.hitboxes,
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sprite"]
    }
}

pub fn sprite_animation_unpack_system(
    query: Query<(Entity, &Handle<SpriteAnimationAsset>), Without<Handle<TextureAtlas>>>,
    sprite_animation_assets: Res<Assets<SpriteAnimationAsset>>,
    mut commands: Commands,
) {
    for (entity, sprite_animation) in query.iter() {
        if let Some(sprite_animation) = sprite_animation_assets.get(sprite_animation) {
            commands
                .entity(entity)
                .insert(sprite_animation.texture_atlas.clone())
                .insert(sprite_animation.parameterized_animation_set.clone());
            if let Some(hitboxes) = &sprite_animation.hitboxes {
                commands.entity(entity).insert(hitboxes.clone());
            }
        }
    }
}
