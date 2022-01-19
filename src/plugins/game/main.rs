use crate::components::{
    FirstPersonHead, FirstPersonSubject, LevelObject, Lookaround, LookaroundDirection, Movement,
    MovementDirection,
};
use crate::states::{FirstPersonControlSettings, GameLevel};
use crate::systems::{activate_physics, deactivate_physics, teardown_game_level};
use bevy::prelude::*;
use bevy_rapier3d::na::{Point3, Vector3};
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
        .add_system_set(
            SystemSet::on_update(GameLevel::Main)
                .with_system(rotate_player_body)
                .with_system(rotate_player_head),
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
    let player_halfheight_raw = 3f32;
    let player_radius_raw = 1f32;
    let player_halfheight_physics = Point3::from(Vector3::y() * player_halfheight_raw);
    commands
        .spawn()
        .insert(FirstPersonSubject)
        .insert(LevelObject)
        .insert(Movement::default())
        .insert(Lookaround::default())
        // The transform is auto-updated by the rigid body
        .insert(Transform::default())
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::capsule(
                -player_halfheight_physics,
                player_halfheight_physics,
                player_radius_raw,
            )
            .into(),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 7.0, 7.0).into(),
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED_X
                | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z)
                .into(),
            ..Default::default()
        })
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Capsule {
                radius: player_radius_raw,
                depth: player_halfheight_raw,
                rings: 3,
                latitudes: 4,
                longitudes: 6,
                uv_profile: bevy::prelude::shape::CapsuleUvProfile::Fixed,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                perceptual_roughness: 1f32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .with_children(|player_body| {
            player_body
                .spawn()
                .insert(FirstPersonHead)
                .insert(
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4))
                        .looking_at(Vec3::new(0f32, 0f32, -1f32), Vec3::Y),
                )
                .insert(LevelObject)
                .insert_bundle(PerspectiveCameraBundle {
                    transform: Transform::default()
                        .looking_at(Vec3::new(0f32, 0f32, -1f32), Vec3::Y),
                    ..Default::default()
                });
        });

    // Enable first person controls
    match fp_control_settings.set(FirstPersonControlSettings::Enabled) {
        Err(_) => {
            panic!("Could not enable First Person Controls while setting up main game level!");
        }
        _ => {}
    };
}

fn rotate_player_head(
    body_query: Query<&Lookaround, With<FirstPersonSubject>>,
    mut head_query: Query<&mut Transform, (With<FirstPersonHead>, Without<FirstPersonSubject>)>,
) {
    let lookaround = match body_query.get_single() {
        Ok(lookaround) => lookaround,
        Err(_) => {
            panic!("Could not find a player with Lookaround component while querying during rotating the player head!");
        }
    };
    match head_query.get_single_mut() {
        Ok(mut head_transform) => {
            match lookaround.up_down() {
                LookaroundDirection::Up(magnitude) => {
                    let (angle, _, _) = head_transform.rotation.to_euler(EulerRot::XYZ);
                    let new_quat = Quat::from_rotation_x(
                        (angle + magnitude * 0.005)
                            .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
                    );
                    head_transform.rotation = new_quat;
                }
                LookaroundDirection::Down(magnitude) => {
                    let (angle, _, _) = head_transform.rotation.to_euler(EulerRot::XYZ);
                    let new_quat = Quat::from_rotation_x(
                        (angle - magnitude * 0.005)
                            .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
                    );
                    head_transform.rotation = new_quat;
                }
                _ => {
                    panic!("Lookaround up_down() was neither Up nor Down!")
                }
            };
        }
        Err(_) => {
            panic!("Could not find a FirstPersonHead with a Transform component while querying for the player's head!");
        }
    };
}

fn rotate_player_body(
    mut query: Query<(&Lookaround, &mut RigidBodyPositionComponent), With<FirstPersonSubject>>,
) {
    match query.get_single_mut() {
        Ok((lookaround, mut body)) => {
            let mut rotation = body.position.rotation;
            match lookaround.left_right() {
                LookaroundDirection::Left(magnitude) => {
                    rotation =
                        rotation.append_axisangle_linearized(&(Vector3::y() * magnitude * 0.001));
                }
                LookaroundDirection::Right(magnitude) => {
                    rotation =
                        rotation.append_axisangle_linearized(&(Vector3::y() * -magnitude * 0.001));
                }
                _ => {
                    panic!("Lookaround left_right() was neither Left nor Right!")
                }
            };
            body.position.rotation = rotation;
        }
        Err(_) => {
            panic!("Could not find a player while querying during rotating the player body!");
        }
    }
}

// fn move_player_body(
//     mut query: Query<
//         (
//             &Movement,
//             &RigidBodyPositionComponent,
//             &mut RigidBodySet,
//         ),
//         With<FirstPersonSubject>,
//     >,
// ) {
//     match query.get_single_mut() {
//         Ok((movement, body_pos, mut body)) => {
//             match movement.left_right() {
//                 MovementDirection::Left(magnitude) => {
//                     body.apply_force_at_point(rb_mprops, force, point);
//                 }
//                 MovementDirection::Right(magnitude) => {
//                     body.apply_force_at_point(rb_mprops, force, point);
//                 }
//                 _ => {
//                     panic!("Movement left_right() was neither Left nor Right!")
//                 }
//             };
//             match movement.forward_back() {
//                 MovementDirection::Forward(magnitude) => {
//                     body.apply_force_at_point(rb_mprops, force, point);
//                 }
//                 MovementDirection::Back(magnitude) => {
//                     body.apply_force_at_point(rb_mprops, force, point);
//                 }
//                 _ => {
//                     panic!("Movement forward_back() was neither Forward nor Back!")
//                 }
//             };
//         }
//         Err(_) => {
//             panic!("Could not find a player while querying during moving the player body!");
//         }
//     }
// }

fn teardown_main_game_level(mut commands: Commands) {
    commands.remove_resource::<AmbientLight>();
}
