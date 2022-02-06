use crate::components::{
    FirstPersonHead, FirstPersonSubject, LevelObject, Lookaround, LookaroundDirection, Movement,
    MovementDirection,
};
use crate::resources::{GameConfig, GameSettings};
use bevy::prelude::*;
use bevy_rapier3d::na::{Point3, Vector3};
use bevy_rapier3d::prelude::*;

pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<GameConfig>,
) {
    let player_config = game_config.player();
    // Add a player
    let player_capsule_total_height = player_config.capsule_height();
    let player_capsule_radius = player_config.capsule_radius();
    let player_halfheight_raw =
        (player_capsule_total_height - (2f32 * player_capsule_radius)) / 2f32;
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
                player_capsule_radius,
            )
            .into(),
            material: ColliderMaterial {
                restitution: 0.15f32,
                friction: 2f32,
                friction_combine_rule: CoefficientCombineRule::Max,
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
                radius: player_capsule_radius,
                depth: 2f32 * player_halfheight_raw,
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
                .insert(LevelObject)
                .insert(Transform::from_rotation(Quat::from_rotation_x(
                    -std::f32::consts::FRAC_PI_4,
                )))
                .insert_bundle(PerspectiveCameraBundle {
                    // Put the camera at the top of the cylinder / bottom of the topmost hemisphere
                    transform: Transform::from_xyz(0f32, player_halfheight_raw, 0f32),
                    ..Default::default()
                });
        });
}

pub fn rotate_player_head(
    body_query: Query<&Lookaround, With<FirstPersonSubject>>,
    mut head_query: Query<&mut Transform, (With<FirstPersonHead>, Without<FirstPersonSubject>)>,
    settings: Res<GameSettings>,
) {
    let lookaround = body_query.get_single().expect("Could not find a player with Lookaround component while querying during rotating the player head!");
    let mut head_transform = head_query.get_single_mut().expect("Could not find a FirstPersonHead with a Transform component while querying for the player's head!");
    match lookaround.up_down() {
        LookaroundDirection::Up(magnitude) => {
            let (angle, _, _) = head_transform.rotation.to_euler(EulerRot::XYZ);
            let new_quat = Quat::from_rotation_x(
                (angle + magnitude * 0.005 * (settings.vertical_sensitivity() as f32 / 5 as f32))
                    .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
            );
            head_transform.rotation = new_quat;
        }
        LookaroundDirection::Down(magnitude) => {
            let (angle, _, _) = head_transform.rotation.to_euler(EulerRot::XYZ);
            let new_quat = Quat::from_rotation_x(
                (angle - magnitude * 0.005 * (settings.vertical_sensitivity() as f32 / 5 as f32))
                    .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
            );
            head_transform.rotation = new_quat;
        }
        _ => {
            panic!("Lookaround up_down() was neither Up nor Down!");
        }
    };
}

pub fn rotate_player_body(
    mut query: Query<(&Lookaround, &mut RigidBodyPositionComponent), With<FirstPersonSubject>>,
    settings: Res<GameSettings>,
) {
    let (lookaround, mut body) = query
        .get_single_mut()
        .expect("Could not find a player while querying during rotating the player body!");

    let mut rotation = body.position.rotation;
    match lookaround.left_right() {
        LookaroundDirection::Left(magnitude) => {
            rotation = rotation.append_axisangle_linearized(
                &(Vector3::y()
                    * magnitude
                    * 0.002
                    * (settings.horizontal_sensitivity() as f32 / 5 as f32)),
            );
        }
        LookaroundDirection::Right(magnitude) => {
            rotation = rotation.append_axisangle_linearized(
                &(Vector3::y()
                    * -magnitude
                    * 0.002
                    * (settings.horizontal_sensitivity() as f32 / 5 as f32)),
            );
        }
        _ => {
            panic!("Lookaround left_right() was neither Left nor Right!")
        }
    };
    body.position.rotation = rotation;
}

pub fn move_player_body(
    mut query: Query<
        (
            &Movement,
            &Transform,
            &mut RigidBodyForcesComponent,
            &RigidBodyVelocityComponent,
        ),
        With<FirstPersonSubject>,
    >,
    game_config: Res<GameConfig>,
) {
    let player_config = game_config.player();
    let (movement, subject_transform, mut body_force, body_velocity) = query
        .get_single_mut()
        .expect("Could not find a player while querying during moving the player body!");
    if body_velocity.linvel.magnitude() < player_config.max_speed() {
        let local_z = subject_transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);
        let left_right_magnitude = match movement.left_right() {
            MovementDirection::Left(magnitude) => -magnitude * player_config.movement_force(),
            MovementDirection::Right(magnitude) => magnitude * player_config.movement_force(),
            _ => {
                panic!("Movement left_right() was neither Left nor Right!")
            }
        };
        let forward_back_magnitude = match movement.forward_back() {
            MovementDirection::Forward(magnitude) => magnitude * player_config.movement_force(),
            MovementDirection::Back(magnitude) => -magnitude * player_config.movement_force(),
            _ => {
                panic!("Movement forward_back() was neither Forward nor Back!")
            }
        };
        body_force.force = (forward * forward_back_magnitude + right * left_right_magnitude).into();
    }
}

pub fn jump_player_body(
    rapier_query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut player_query: Query<
        (&GlobalTransform, &mut RigidBodyForcesComponent),
        With<FirstPersonSubject>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    game_config: Res<GameConfig>,
) {
    let player_config = game_config.player();
    let (player_transform, mut body_forces) = player_query
        .get_single_mut()
        .expect("Could not find a player while querying during making the player body jump!");

    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
    let mut player_global_position = player_transform.translation;
    player_global_position.y -= (player_config.capsule_height() / 2f32) + 0.01;
    let ray = Ray::new(
        player_global_position.into(),
        Vec3::new(0.0, -1.0, 0.0).into(),
    );
    let max_toi = 0.02;
    let solid = true;
    let groups = InteractionGroups::all();
    let filter = None;

    if let Some(_) =
        rapier_query_pipeline.cast_ray(&collider_set, &ray, max_toi, solid, groups, filter)
    {
        let mut jump_vector = vector![0f32, 0f32, 0f32];
        if keyboard_input.just_pressed(KeyCode::Space) {
            jump_vector.y = player_config.jump_force();
        }
        for gamepad in gamepads.iter().cloned() {
            if gamepad_buttons.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
                jump_vector.y = player_config.jump_force();
            }
        }
        body_forces.force = (body_forces.force as Vector3<f32>) + jump_vector;
    }
}
