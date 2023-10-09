use std::default;

use bevy::{prelude::*, window::WindowPlugin, DefaultPlugins};

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.25;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nicole Bevy Template".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
