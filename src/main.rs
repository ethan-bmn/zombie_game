use bevy::prelude::*;
use bevy::input::mouse::*;
use crate::player::*;
use crate::setup::*;
mod player;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor{
            width: 1280.0,
            height: 720.0,
            position: WindowPosition::Automatic,
            title: "zombie_game.rs".to_string(),
            ..default()
        })
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn update(mut _commands: Commands,
          mut player_query: Query<&mut Transform, With<Player>>,
          windows: Res<Windows>) {
    for _transform in player_query.iter_mut() {
        let window = windows.get_primary().unwrap();
        if let Some(_position) = window.cursor_position() {

        }
    }
}