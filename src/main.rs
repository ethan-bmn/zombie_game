use bevy::prelude::*;
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
        .add_startup_system_to_stage(StartupStage::Startup, setup)
        .add_startup_system(Player::setup)
        .add_system(Player::update)
        .insert_resource(PlayerRes)
        .run();
}
