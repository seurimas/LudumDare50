use super::TerrainBlock;
use crate::prelude::*;

#[derive(Default, Component, Debug, Clone, Reflect, Inspectable)]
#[reflect(Component)]
pub struct GroundedState {
    grounded_for: f32,
    air_for: f32,
    half_width: f32,
    half_height: f32,
    feet_depth: f32,
}

impl GroundedState {
    pub fn new(half_width: f32, half_height: f32, feet_depth: f32) -> Self {
        GroundedState {
            half_width,
            half_height,
            grounded_for: 0.0,
            air_for: 0.0,
            feet_depth,
        }
    }
    pub fn on_the_ground(&self) -> bool {
        self.grounded_for > 0.0
    }

    pub fn lift_off(&mut self) {
        self.grounded_for = 0.0;
        self.air_for = 0.2;
    }
}

pub fn grounded_system(
    query_pipeline: Res<QueryPipeline>,
    time: Res<Time>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut player_query: Query<(&RigidBodyPositionComponent, &mut GroundedState)>,
    terrain_query: Query<&TerrainBlock>,
) {
    for (position, mut grounded) in player_query.iter_mut() {
        if grounded.air_for > 0.0 {
            grounded.air_for -= time.delta_seconds();
            continue;
        }
        grounded.grounded_for -= time.delta_seconds();
        let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
        let shape = Cuboid::new(Vec2::new(grounded.half_width - 0.1, grounded.half_height).into());
        let mut shape_pos = position.0.position.translation.clone();
        shape_pos.y -= grounded.feet_depth;
        let groups = InteractionGroups::all();
        let filter = None;
        query_pipeline.intersections_with_shape(
            &collider_set,
            &shape_pos.into(),
            &shape,
            groups,
            filter,
            |handle| {
                if terrain_query.get(handle.entity()).is_ok() {
                    grounded.grounded_for = 0.1;
                    false
                } else {
                    true
                }
            },
        )
    }
}
