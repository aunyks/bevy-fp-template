use bevy::ecs::component::Component;
mod lookaround;
mod movement;

pub use self::lookaround::*;
pub use self::movement::*;

/// This component is used to define an entity that can be controlled by the player.
/// It should be used on an entity that also has a [`Movement`](crate::components::Movement) and /
/// or [`Lookaround`](crate::components::Lookaround) component.
///
/// Note: There must only be _one_ entity with this component
/// whenever [`FirstPersonControlSettings`](crate::states::FirstPersonControlSettings) is enabled, otherwise a
/// panic may occur.
#[derive(Component)]
pub struct FirstPersonSubject;

#[derive(Component)]
pub struct FirstPersonHead;

/// This component is used to define an entity that exists in one level at a time. It should
/// be set up and torn down with every level transition.
#[derive(Component)]
pub struct LevelObject;
