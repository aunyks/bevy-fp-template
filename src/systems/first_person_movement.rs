use crate::components::*;
use bevy::prelude::*;

/// This function listens for keyboard and gamepad events and
/// updates the [`Movement`](crate::components::Movement) component of an entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject)
/// component.
///
/// The function listens for WASD and arrow key presses on the keyboard, and
/// it listens for left stick events on a gamepad.
///
/// Note: This function will panic if there is either _no_ entity
/// with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component or if there are more than one
/// entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component.
pub fn first_person_movement(
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Movement, With<FirstPersonSubject>>,
) {
    // Set defaults
    let mut left_right = MovementDirection::Right(0f32);
    let mut forward_back = MovementDirection::Forward(0f32);

    // Process Keyboard input
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        left_right = MovementDirection::Left(1f32);
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        left_right = MovementDirection::Right(1f32);
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        forward_back = MovementDirection::Back(1f32);
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        forward_back = MovementDirection::Forward(1f32);
    }

    // Process gamepad input because they have precedence
    // over keyboard input
    for gamepad in gamepads.iter().cloned() {
        if let Some(magnitude) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX)) {
            if magnitude != 0f32 {
                left_right = if magnitude > 0f32 {
                    MovementDirection::Right(magnitude)
                } else {
                    MovementDirection::Left(magnitude.abs())
                };
            }
        }
        if let Some(magnitude) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY)) {
            if magnitude != 0f32 {
                forward_back = if magnitude > 0f32 {
                    MovementDirection::Forward(magnitude)
                } else {
                    MovementDirection::Back(magnitude.abs())
                };
            }
        }
    }

    match query.get_single_mut() {
        Ok(mut movement) => {
            movement.set_left_right(left_right);
            movement.set_forward_back(forward_back);
        }
        Err(_) => {
            panic!("Could not find a player when querying using Movement component!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement_idle() {
        // Setup our world
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());

        // Create an entity with a movement component
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        // Create the args that will be requested by the system we're testing
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let keyboard_input: Input<KeyCode> = Input::default();
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        // Add them
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        // Add the system
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        // Step
        schedule.run_once(&mut world);

        // Sanity check that everything works okay
        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(0f32)
            )
        );
    }

    #[test]
    fn test_player_movement_w() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press W
        keyboard_input.press(KeyCode::W);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_s() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press S
        keyboard_input.press(KeyCode::S);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Back(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_a() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press A
        keyboard_input.press(KeyCode::A);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Left(1f32),
                MovementDirection::Forward(0f32)
            )
        );
    }

    #[test]
    fn test_player_movement_d() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press D
        keyboard_input.press(KeyCode::D);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(1f32),
                MovementDirection::Forward(0f32)
            )
        );
    }

    #[test]
    fn test_player_movement_wd() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press W and D
        keyboard_input.press(KeyCode::W);
        keyboard_input.press(KeyCode::D);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(1f32),
                MovementDirection::Forward(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_sa() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press S and A
        keyboard_input.press(KeyCode::S);
        keyboard_input.press(KeyCode::A);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Left(1f32),
                MovementDirection::Back(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_press_release() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut player_query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press W
        keyboard_input.press(KeyCode::W);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = player_query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(1f32)
            )
        );

        // Get the keyboard input to release W
        let kbd_input_resource = world.get_resource_mut::<Input<KeyCode>>();
        let kbd_input = kbd_input_resource.unwrap().into_inner();
        kbd_input.release(KeyCode::W);
        schedule.run_once(&mut world);

        let (movement, _) = player_query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(0f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrow_up() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press Up
        keyboard_input.press(KeyCode::Up);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrow_down() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press Down
        keyboard_input.press(KeyCode::Down);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Back(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrow_left() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press Left
        keyboard_input.press(KeyCode::Left);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Left(1f32),
                MovementDirection::Forward(0f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrow_right() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press Right
        keyboard_input.press(KeyCode::Right);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(1f32),
                MovementDirection::Forward(0f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrows_up_right() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press Up and Right
        keyboard_input.press(KeyCode::Up);
        keyboard_input.press(KeyCode::Right);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(1f32),
                MovementDirection::Forward(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrows_down_left() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press Down and Left
        keyboard_input.press(KeyCode::Down);
        keyboard_input.press(KeyCode::Left);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Left(1f32),
                MovementDirection::Back(1f32)
            )
        );
    }

    #[test]
    fn test_player_movement_arrow_press_release() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());
        let player_entity = world
            .spawn()
            .insert(Movement::default())
            .insert(FirstPersonSubject)
            .id();
        let mut player_query = world.query::<(&mut Movement, With<FirstPersonSubject>)>();
        let mut keyboard_input: Input<KeyCode> = Input::default();
        // Press W
        keyboard_input.press(KeyCode::Up);
        let gamepads = Gamepads::default();
        let axes: Axis<GamepadAxis> = Axis::default();
        world.insert_resource(keyboard_input);
        world.insert_resource(gamepads);
        world.insert_resource(axes);
        schedule.add_system_to_stage("update", first_person_movement.label("first"));
        schedule.run_once(&mut world);

        let (movement, _) = player_query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(1f32)
            )
        );

        // Get the keyboard input to release up
        let kbd_input_resource = world.get_resource_mut::<Input<KeyCode>>();
        let kbd_input = kbd_input_resource.unwrap().into_inner();
        kbd_input.release(KeyCode::Up);
        schedule.run_once(&mut world);

        let (movement, _) = player_query.get_mut(&mut world, player_entity).unwrap();
        assert_eq!(
            movement.into_inner(),
            &mut Movement::from_components(
                MovementDirection::Right(0f32),
                MovementDirection::Forward(0f32)
            )
        );
    }
}
