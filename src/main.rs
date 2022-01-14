use bevy::log::LogSettings;
use bevy::prelude::*;

use bevy_rapier3d::physics::TimestepMode;
use bevy_rapier3d::prelude::*;

use plugins::game::MainGameLevel;
use plugins::FirstPersonControlPlugin;
use resources::{GameConfig, GameSettings};
use states::{FirstPersonControlSettings, GameLevel};

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

fn main() {
    let game_config = GameConfig::default();

    App::new()
        // Configure log plugin (added by DefaultPlugins)
        .insert_resource(LogSettings {
            level: game_config.log_level(),
            filter: game_config.log_filter(),
        })
        .insert_resource(WindowDescriptor {
            title: game_config.window_title(),
            width: 800.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(game_config.clone())
        .insert_resource(GameSettings::default())
        // Enable First Person controls
        .add_state(FirstPersonControlSettings::Disabled)
        .add_plugin(FirstPersonControlPlugin)
        .add_state(GameLevel::Main)
        // Configure and add physics
        .insert_resource(RapierConfiguration {
            gravity: Vector::y() * -9.81,
            scale: 1.0,
            // Turn off the sim to start
            physics_pipeline_active: false,
            query_pipeline_active: false,
            timestep_mode: TimestepMode::VariableTimestep,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(MainGameLevel)
        .run();
}
