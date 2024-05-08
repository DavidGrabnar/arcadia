use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

const PLAYGROUND_WIDTH: f32 = 10.0;
const PLAYGROUND_HEIGHT: f32 = 10.0;

#[derive(Component)]
struct Player {}

/// initialize scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            // 6 world units per window height.
            scaling_mode: ScalingMode::FixedVertical(6.0),
            ..default()
        }
            .into(),
        transform: Transform::from_xyz(0.0, 3.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // playing ground - for debugging
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, PLAYGROUND_HEIGHT / 2.0),
        mesh: meshes.add(Plane3d::default().mesh().size(PLAYGROUND_WIDTH / 2.0, PLAYGROUND_HEIGHT / 2.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // player - behind Z0.0
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.3, 0.4, 0.8)),
        transform: Transform::from_xyz(0.0, 0.5, -1.5),
        ..default()
    }).insert(Player{});
    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}
