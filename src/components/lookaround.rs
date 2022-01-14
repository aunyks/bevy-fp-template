use bevy::ecs::component::Component;

const LOOKAROUND_DIRECTION_MARGIN: f32 = 0.01f32;

/// This enum defines the direction in which the player wants an entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component to look.
/// The inner value specifies the magnitude (or speed) with which the subject
/// should change its orientation.
#[derive(Debug)]
pub enum LookaroundDirection {
    Left(f32),
    Right(f32),
    Up(f32),
    Down(f32),
}

impl PartialEq for LookaroundDirection {
    fn eq(&self, other: &Self) -> bool {
        use LookaroundDirection::*;

        match (self, other) {
            (&Left(ref mag_a), &Left(ref mag_b)) => {
                (mag_a - mag_b).abs() < LOOKAROUND_DIRECTION_MARGIN
            }
            (&Right(ref mag_a), &Right(ref mag_b)) => {
                (mag_a - mag_b).abs() < LOOKAROUND_DIRECTION_MARGIN
            }
            (&Up(ref mag_a), &Up(ref mag_b)) => (mag_a - mag_b).abs() < LOOKAROUND_DIRECTION_MARGIN,
            (&Down(ref mag_a), &Down(ref mag_b)) => {
                (mag_a - mag_b).abs() < LOOKAROUND_DIRECTION_MARGIN
            }
            _ => false,
        }
    }
}
impl Eq for LookaroundDirection {}

/// This struct defines how an entity with a [`FirstPersonSubject`](crate::components::FirstPersonSubject) component should change its orientation.
#[derive(Component, PartialEq, Eq, Debug)]
pub struct Lookaround {
    left_right: LookaroundDirection,
    up_down: LookaroundDirection,
}

impl Default for Lookaround {
    fn default() -> Self {
        Lookaround {
            left_right: LookaroundDirection::Right(0f32),
            up_down: LookaroundDirection::Up(0f32),
        }
    }
}

impl Lookaround {
    #[allow(dead_code)]
    pub fn from_components(left_right: LookaroundDirection, up_down: LookaroundDirection) -> Self {
        Lookaround {
            left_right: left_right,
            up_down: up_down,
        }
    }

    pub fn set_left_right(&mut self, direction: LookaroundDirection) {
        self.left_right = direction;
    }

    pub fn set_up_down(&mut self, direction: LookaroundDirection) {
        self.up_down = direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookarounddirection_eq_within_margin() {
        let lkarnd_left_a = LookaroundDirection::Left(0f32);
        let lkarnd_left_b = LookaroundDirection::Left(0.001);
        assert_eq!(lkarnd_left_a, lkarnd_left_b);
    }

    #[test]
    fn test_lookarounddirection_eq_at_margin() {
        let lkarnd_left_a = LookaroundDirection::Left(0f32);
        let lkarnd_left_b = LookaroundDirection::Left(0.01);
        assert_ne!(lkarnd_left_a, lkarnd_left_b);
    }

    #[test]
    fn test_lookarounddirection_eq_out_of_margin() {
        let lkarnd_left_a = LookaroundDirection::Left(0f32);
        let lkarnd_left_b = LookaroundDirection::Left(0.02);
        assert_ne!(lkarnd_left_a, lkarnd_left_b);
    }

    #[test]
    fn test_lookarounddirection_eq_diff_direction() {
        let lkarnd_left = LookaroundDirection::Left(0f32);
        let lkarnd_right = LookaroundDirection::Right(0f32);
        assert_ne!(lkarnd_left, lkarnd_right);
    }

    #[test]
    fn test_lookarounddirection_eq_diff_direction_diff_margin() {
        let lkarnd_left = LookaroundDirection::Left(0f32);
        let lkarnd_right = LookaroundDirection::Right(0.001f32);
        assert_ne!(lkarnd_left, lkarnd_right);
    }
}
