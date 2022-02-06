use crate::components::LevelObject;
use crate::states::{FirstPersonControlSettings, GameLevel};
use crate::systems::pausing::{pause_game, resume_game};
use crate::systems::player::{
    add_player, jump_player_body, move_player_body, rotate_player_body, rotate_player_head,
};
use crate::systems::{activate_physics, deactivate_physics, teardown_game_level};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// This plugin manages gameplay for the main game level
pub struct MainGameLevel;

impl Plugin for MainGameLevel {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameLevel::Main)
                .with_system(activate_physics)
                .with_system(setup_level)
                .with_system(add_player),
        )
        .add_system_set(SystemSet::on_pause(GameLevel::Main).with_system(deactivate_physics))
        .add_system_set(
            SystemSet::on_resume(GameLevel::Main)
                .with_system(activate_physics)
                .with_system(resume_game),
        )
        .add_system_set(
            SystemSet::on_update(GameLevel::Main)
                .with_system(rotate_player_body)
                .with_system(rotate_player_head)
                .with_system(pause_game)
                // Make sure jump system runs after movement to prevent
                // the bug where the player can't jump without moving at the same time
                .with_system(move_player_body.label("move-player-body"))
                .with_system(jump_player_body.after("move-player-body")),
        )
        .add_system_set(
            SystemSet::on_exit(GameLevel::Main)
                .with_system(teardown_game_level)
                .with_system(teardown_main_game_level)
                .with_system(deactivate_physics),
        );
    }
}

// Set up physics and graphics
fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fp_control_settings: ResMut<State<FirstPersonControlSettings>>,
) {
    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1, 100.0).into(),
        ..Default::default()
    };
    commands.spawn_bundle(collider).insert(LevelObject);

    /* Create the bouncing ball. */
    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 10.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.5).into(),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(LevelObject)
        .insert(Transform::default())
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::UVSphere {
                radius: 0.5,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..Default::default()
            }),
            ..Default::default()
        });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1f32,
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane { size: 10.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1f32,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0f32, 0.1, 0f32),
            ..Default::default()
        })
        .insert(LevelObject);

    // Enable first person controls
    fp_control_settings
        .set(FirstPersonControlSettings::Enabled)
        .expect("Could not enable First Person Controls while setting up main game level!");
}

fn teardown_main_game_level(
    mut commands: Commands,
    mut fp_control_settings: ResMut<State<FirstPersonControlSettings>>,
) {
    commands.remove_resource::<AmbientLight>();
    // Enable first person controls
    fp_control_settings
        .set(FirstPersonControlSettings::Disabled)
        .expect("Could not disable First Person Controls while tearing down main game level!");
}
