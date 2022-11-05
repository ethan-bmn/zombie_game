use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	#[bundle]
	pub sprite: SpriteBundle
}

#[derive(Component)]
pub struct Player;