mod entity;
mod grounded;

pub use self::entity::*;
pub use self::grounded::*;

use crate::prelude::*;

pub fn spawn_parallax(mut commands: Commands, assets: Res<AssetServer>) {
    let mut transform = Transform::default();
    transform.scale.x = 2.0;
    transform.translation.x = 0.0;
    transform.translation.y = 256.0;
    transform.translation.z = 0.1;
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("sprites/Background.png"),
            transform,
            ..Default::default()
        })
        .insert(Parallax);
}

pub fn spawn_bedrock(mut commands: Commands, assets: Res<AssetServer>) {
    let position = Vec2::new(0.0, -2.0);
    let width = 2048.0 / 32.0;
    let height = 2.0;
    let mut transform = Transform::default();
    transform.translation.z = 0.2;
    commands
        .spawn_bundle(terrain_collider_bundle(width, height))
        .insert_bundle(terrain_rigid_body_bundle(position))
        .insert_bundle(SpriteBundle {
            texture: assets.load("sprites/Foreground.png"),
            transform,
            ..Default::default()
        })
        .insert(TerrainBlock)
        .insert(RigidBodyPositionSync::Discrete);
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_bedrock)
            .add_startup_system(spawn_parallax)
            .add_system(grounded_system)
            .register_type::<TerrainBlock>()
            .register_inspectable::<TerrainBlock>();
    }
}
