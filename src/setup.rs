use bevy::prelude::*;
use crate::player::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(Camera2dBundle::default());
	commands.spawn_bundle( PlayerBundle {
		player: Player,
		sprite: SpriteBundle{
			texture: asset_server.load("red.png").into(),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			global_transform: GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 0.0)).into(),
			..default()
		}
	}
	);
}