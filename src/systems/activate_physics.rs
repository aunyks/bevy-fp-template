use bevy::ecs::system::ResMut;
use bevy_rapier3d::physics::RapierConfiguration;

/// Find the [`RapierConfiguration`](bevy_rapier3d::physics::RapierConfiguration)
/// resource and activate its physics and query pipelines so the simulation can start.
///
/// It may panic if there's no [`RapierConfiguration`](bevy_rapier3d::physics::RapierConfiguration) resource.
pub fn activate_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
}
