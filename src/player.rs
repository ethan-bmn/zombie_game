use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	#[bundle]
	pub sprite: SpriteBundle
}

#[derive(Component)]
pub struct Player;

pub struct PlayerRes(pub Entity);

pub trait Manage {
	fn setup(commands: Commands,
			 asset_server: Res<AssetServer>);

	fn update(player_query: Query<&mut Transform, With<Player>>,
			  windows: Res<Windows>);
}

impl Manage for Player {
	fn setup(mut commands: Commands,
			 asset_server: Res<AssetServer>) {
		let player = commands.spawn_bundle(PlayerBundle {
			player: Player,
			sprite: SpriteBundle {
				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				texture: asset_server.load("red.png"),
				..default()
			}
		}).id();
		commands.insert_resource(PlayerRes(player));
	}

	fn update(mut player_query: Query<&mut Transform, With<Player>>,
			  windows: Res<Windows>) {
		for mut transform in player_query.iter_mut() {
			let window = windows.get_primary().unwrap();
			if let Some(_position) = window.cursor_position() {
				transform.rotation = Quat::from_rotation_z(get_angle(Vec2{
					x: transform.translation.x,
					y: transform.translation.y
				},
				 Vec2{
					 x: window.cursor_position().unwrap().x-window.width()/2.0,
					 y: window.cursor_position().unwrap().y-window.height()/2.0
				 }));
			}
		}
	}
}

fn get_angle(a: Vec2, b: Vec2) -> f32 {
	let op = b.y - a.y;
	let adj = b.x - a.x;
	return (op/adj).atan();
}