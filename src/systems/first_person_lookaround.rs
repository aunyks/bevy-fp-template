use crate::components::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// This function listens for mouse and gamepad events and
/// updates the [`Lookaround`](crate::components::Lookaround) component of an entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject)
/// component.
///
/// The function listens for mouse movement, and it listens for right stick events
/// on a gamepad.
///
/// Note: This function will panic if there is either _no_ entity
/// with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component or if there are more than one
/// entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component.
pub fn first_person_lookaround(
    mut mouse_motion_events: EventReader<MouseMotion>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Lookaround, With<FirstPersonSubject>>,
) {
    let mut left_right = LookaroundDirection::Right(0f32);
    let mut up_down = LookaroundDirection::Up(0f32);

    for mouse_motion_event in mouse_motion_events.iter() {
        let delta_x = mouse_motion_event.delta.x;
        let delta_y = mouse_motion_event.delta.y;
        left_right = if delta_x > 0f32 {
            LookaroundDirection::Right(delta_x)
        } else if delta_x < 0f32 {
            LookaroundDirection::Left(delta_x.abs())
        } else {
            left_right
        };
        up_down = if delta_y > 0f32 {
            LookaroundDirection::Down(delta_y)
        } else if delta_y < 0f32 {
            LookaroundDirection::Up(delta_y.abs())
        } else {
            up_down
        };
    }

    // Process gamepad input because they have precedence
    // over keyboard input
    for gamepad in gamepads.iter().cloned() {
        if let Some(magnitude) = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickX)) {
            if magnitude != 0f32 {
                left_right = if magnitude > 0f32 {
                    LookaroundDirection::Right(magnitude * 11f32)
                } else {
                    LookaroundDirection::Left(magnitude.abs() * 11f32)
                };
            }
        }
        if let Some(magnitude) = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickY)) {
            if magnitude != 0f32 {
                up_down = if magnitude > 0f32 {
                    LookaroundDirection::Up(magnitude * 6.5)
                } else {
                    LookaroundDirection::Down(magnitude.abs() * 6.5)
                };
            }
        }
    }

    let mut lookaround = query
        .get_single_mut()
        .expect("Could not find a player when querying using Lookaround component!");
    lookaround.set_left_right(left_right);
    lookaround.set_up_down(up_down);
}
