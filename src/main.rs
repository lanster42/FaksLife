use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(movement)
        .run();
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();

    let speed = 2.0;
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += speed;
    }
}

#[derive(Component)]
struct Player;
