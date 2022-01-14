use crate::components::{FirstPersonSubject, LevelObject, Lookaround, Movement};
use crate::states::{FirstPersonControlSettings, GameLevel};
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
                .with_system(setup_level),
        )
        // .add_system_set(
        //     SystemSet::on_update(GameLevel::Main)
        //         .with_system(first_person_movement)
        //         .with_system(first_person_lookaround),
        // )
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
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 10.0, 0.0).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5).into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
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
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(LevelObject);

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

    // Add a player
    commands
        .spawn()
        .insert(LevelObject)
        .insert(Movement::default())
        .insert(Lookaround::default())
        .insert(Transform {
            translation: Vec3::new(0f32, 0f32, 0f32),
            ..Default::default()
        })
        .insert(FirstPersonSubject);

    // Enable first person controls
    match fp_control_settings.set(FirstPersonControlSettings::Enabled) {
        Err(_) => {
            panic!("Could not enable First Person Controls while setting up main game level!");
        }
        _ => {}
    };
}

fn teardown_main_game_level(mut commands: Commands) {
    commands.remove_resource::<AmbientLight>();
}
