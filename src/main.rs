use std::f32::consts::PI;

use bevy::{prelude::*, window::WindowPlugin, DefaultPlugins};

#[derive(Component)]
struct Rotatable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.25;

    let my_mesh = meshes.add(Mesh::from(shape::Torus::default()));
    let my_material = materials.add(Color::WHITE.into());

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: my_mesh,
            material: my_material,
            ..default()
        },
        Rotatable,
    ));
}

fn system_continuous_rotate(mut rotatables: Query<(&mut Rotatable, &mut Transform)>) {
    for (_rotatable, mut transform) in rotatables.iter_mut() {
        transform.rotate_x(PI * 0.001);
        transform.rotate_y(PI * 0.001);
    }
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
        .add_systems(Update, system_continuous_rotate)
        .run();
}
