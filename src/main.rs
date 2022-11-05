use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor{
            width: 1280.0,
            height: 720.0,
            position: WindowPosition::Automatic,
            title: "".to_string(),
            ..default()
        })
        .run();
}
