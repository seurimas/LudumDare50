mod entity;
mod grounded;
pub use self::entity::*;
pub use self::grounded::*;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::RegisterInspectable;

pub fn spawn_bedrock(mut commands: Commands) {
    spawn_terrain(&mut commands, Vec2::new(0.0, -5.0), 500.0, 5.0);
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_bedrock)
            .add_system(grounded_system)
            .register_type::<TerrainBlock>()
            .register_inspectable::<TerrainBlock>();
    }
}
