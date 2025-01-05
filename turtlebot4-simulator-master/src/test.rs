use bevy::prelude::*;
extern crate bevy;
use bevy::input::keyboard::KeyCode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(keyboard_input_system.system())
        .run();
}

fn setup() {
    println!("Press keys to see input!");
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        println!("W key was pressed");
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        println!("A key was pressed");
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        println!("S key was pressed");
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        println!("D key was pressed");
    }

    // if keyboard_input.released(KeyCode::Space) {
    //     println!("Space key was released");
    // }
}
