use bevy::prelude::*;
use crate::player::*;
use crate::setup::*;
mod player;
mod setup;

#[derive(Default, Copy, Clone)]
struct Angle(f32);

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
        .insert_resource(Angle(0.0))
        .run();
}

fn update(keys: Res<Input<KeyCode>>,
          mut player_query: Query<&mut Transform, With<Player>>,
          windows: Res<Windows>, mut angle: ResMut<Angle>) {
    for mut transform in player_query.iter_mut() {
        let window = windows.get_primary().unwrap();
        if let Some(_position) = window.cursor_position() {
            if keys.pressed(KeyCode::Right) {
                angle.0 -= 1.0;
            }
            if keys.pressed(KeyCode::Left) {
                angle.0 += 1.0;
            }
            println!("{}", get_angle(Vec2{
                x: transform.translation.x,
                y: transform.translation.y
            }, window.cursor_position().unwrap()));

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

fn get_angle(a: Vec2, b: Vec2) -> f32 {
    let op = b.y - a.y;
    let adj = b.x - a.x;
    if op == 0.0 ||adj == 0.0 {
        return 0.0;
    }
    return (op/adj).atan();
}