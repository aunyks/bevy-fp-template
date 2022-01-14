use bevy::ecs::component::Component;

const MOVEMENT_DIRECTION_MARGIN: f32 = 0.01f32;

/// This enum defines the direction in which the player wants an entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component to move.
/// The inner value specifies the magnitude (or speed) with which the subject
/// should change its position.
#[derive(Debug)]
pub enum MovementDirection {
    Left(f32),
    Right(f32),
    Forward(f32),
    Back(f32),
}

impl PartialEq for MovementDirection {
    fn eq(&self, other: &Self) -> bool {
        use MovementDirection::*;

        match (self, other) {
            (&Left(ref mag_a), &Left(ref mag_b)) => {
                (mag_a - mag_b).abs() < MOVEMENT_DIRECTION_MARGIN
            }
            (&Right(ref mag_a), &Right(ref mag_b)) => {
                (mag_a - mag_b).abs() < MOVEMENT_DIRECTION_MARGIN
            }
            (&Forward(ref mag_a), &Forward(ref mag_b)) => {
                (mag_a - mag_b).abs() < MOVEMENT_DIRECTION_MARGIN
            }
            (&Back(ref mag_a), &Back(ref mag_b)) => {
                (mag_a - mag_b).abs() < MOVEMENT_DIRECTION_MARGIN
            }
            _ => false,
        }
    }
}
impl Eq for MovementDirection {}

/// This struct defines how an entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component should change its position.
#[derive(Component, PartialEq, Eq, Debug)]
pub struct Movement {
    left_right: MovementDirection,
    forward_back: MovementDirection,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            left_right: MovementDirection::Right(0f32),
            forward_back: MovementDirection::Forward(0f32),
        }
    }
}

impl Movement {
    #[allow(dead_code)]
    pub fn from_components(left_right: MovementDirection, forward_back: MovementDirection) -> Self {
        Movement {
            left_right: left_right,
            forward_back: forward_back,
        }
    }

    pub fn set_left_right(&mut self, direction: MovementDirection) {
        self.left_right = direction;
    }

    pub fn set_forward_back(&mut self, direction: MovementDirection) {
        self.forward_back = direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movementdirection_eq_within_margin() {
        let mvmnt_left_a = MovementDirection::Left(0f32);
        let mvmnt_left_b = MovementDirection::Left(0.001);
        assert_eq!(mvmnt_left_a, mvmnt_left_b);
    }

    #[test]
    fn test_movementdirection_eq_at_margin() {
        let mvmnt_left_a = MovementDirection::Left(0f32);
        let mvmnt_left_b = MovementDirection::Left(0.01);
        assert_ne!(mvmnt_left_a, mvmnt_left_b);
    }

    #[test]
    fn test_movementdirection_eq_out_of_margin() {
        let mvmnt_left_a = MovementDirection::Left(0f32);
        let mvmnt_left_b = MovementDirection::Left(0.02);
        assert_ne!(mvmnt_left_a, mvmnt_left_b);
    }

    #[test]
    fn test_movementdirection_eq_diff_direction() {
        let mvmnt_left = MovementDirection::Left(0f32);
        let mvmnt_right = MovementDirection::Right(0f32);
        assert_ne!(mvmnt_left, mvmnt_right);
    }

    #[test]
    fn test_movementdirection_eq_diff_direction_diff_margin() {
        let mvmnt_left = MovementDirection::Left(0f32);
        let mvmnt_right = MovementDirection::Right(0.001f32);
        assert_ne!(mvmnt_left, mvmnt_right);
    }
}
