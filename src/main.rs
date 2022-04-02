use animation::AnimationPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;
use minions::MinionsPlugin;
use player::PlayerPlugin;
use terrain::TerrainPlugin;

mod ai;
mod animation;
mod base_bundles;
mod minions;
mod player;
mod terrain;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            far: 1000.0,
            depth_calculation: bevy::render::camera::DepthCalculation::ZDifference,
            scale: 0.5,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    });
}

fn display_rapier_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(AnimationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MinionsPlugin)
        .add_plugin(TerrainPlugin)
        .add_system(display_rapier_events)
        .insert_resource(RapierConfiguration {
            gravity: Vector::y() * -98.1,
            scale: 10.0,
            ..Default::default()
        })
        .run();
}
