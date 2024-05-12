use bevy::{prelude::*};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Game{enemy_direction : -1.0})
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (move_player, move_enemy, shoot_bullet, move_bullet, check_collision))
        .run();
}

const PLAYGROUND_WIDTH: f32 = 10.0;
const PLAYGROUND_HEIGHT: f32 = 10.0;
const PLAYER_SPEED: f32 = 3.0;
const BULLET_SPEED: f32 = 5.0;
const ENEMY_COUNT: i32 = 20;
const ENEMY_PER_ROW: i32 = 5;
const ENEMY_SPEED: f32 = 3.0;

#[derive(Component)]
struct Player {
    health: i32
}

#[derive(Component)]
struct Enemy {}

#[derive(Component)]
struct Bullet {}

#[derive(Resource, Default)]
struct Game {
    enemy_direction: f32
}

#[derive(Component)]
struct HealthUi;

/// initialize scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: PerspectiveProjection {

            ..default()
        }
            .into(),
        transform: Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // playing ground - for debugging
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, PLAYGROUND_HEIGHT / 2.0),
        mesh: meshes.add(Plane3d::default().mesh().size(PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // player - behind Z0.0
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::rgb(0.3, 0.4, 0.8)),
        transform: Transform::from_xyz(0.0, 0.5, -1.5),
        ..default()
    }).insert(Player{ health: 3 });

    // enemies
    for i in 0..ENEMY_COUNT {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::splat(0.5))),
            material: materials.add(Color::rgb(0.8, 0.4, 0.3)),
            transform: Transform::from_xyz(
                PLAYGROUND_WIDTH / 2.0 / ENEMY_PER_ROW as f32 * (i % ENEMY_PER_ROW) as f32 - PLAYGROUND_WIDTH / 2.0 + 1.0,
                0.5,
                PLAYGROUND_HEIGHT - (PLAYGROUND_HEIGHT / 2.0 / (ENEMY_COUNT as f32 / ENEMY_PER_ROW as f32) * (i / ENEMY_PER_ROW) as f32) - 1.0),
            ..default()
        }).insert(Enemy{});

        println!("{:?}", Transform::from_xyz(
            PLAYGROUND_WIDTH / 2.0 / ENEMY_PER_ROW as f32 * (i % ENEMY_PER_ROW) as f32 - PLAYGROUND_WIDTH / 2.0 + 1.0,
            0.5,
            PLAYGROUND_HEIGHT - (PLAYGROUND_HEIGHT / 2.0 / (ENEMY_COUNT as f32 / ENEMY_PER_ROW as f32) * (i / ENEMY_PER_ROW) as f32) - 1.0))
    }

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

fn setup_ui(
    mut commands: Commands,
    p: Query<&Player>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Health: {}", p.get_single().map(|p| p.health).unwrap_or(0)),
                                    TextStyle {
                                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                                HealthUi
                            ));
                        });
                });
            });
}

fn move_player(
    mut p: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut t = p.single_mut();

    let dir = if keys.pressed(KeyCode::ArrowLeft) {
        1.0
    } else if keys.pressed(KeyCode::ArrowRight) {
        -1.0
    } else {
        0.0
    };
    t.translation.x =
        (t.translation.x + PLAYER_SPEED * time.delta_seconds() * dir)
            .clamp(- PLAYGROUND_WIDTH / 2.0 + 0.5, PLAYGROUND_WIDTH / 2.0 - 0.5);
}

fn move_enemy(
    mut e: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
    mut g: ResMut<Game>
) {
    let mut flip_dir = false;
    for mut t in e.iter_mut() {
        let prev = t.translation.x;
        t.translation.x = prev + ENEMY_SPEED * time.delta_seconds() * g.enemy_direction;

        if t.translation.x.abs() >= PLAYGROUND_WIDTH / 2.0 - 0.5 {
            flip_dir = true;
        }
    }
    if flip_dir {
        g.enemy_direction *= -1.0;
    }
}

fn shoot_bullet(
    mut evs: EventReader<KeyboardInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    p: Query<&Transform, With<Player>>
) {
    let t = p.single();

    let mut bullet_t = t.clone();
    bullet_t.translation.z += 1.0;
    for event in evs.read() {
        if event.state == ButtonState::Pressed && event.key_code == KeyCode::Space {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.3)),
                material: materials.add(Color::rgb(0.9, 0.8, 0.1)),
                transform: bullet_t,
                ..default()
            }).insert(Bullet{});
        }
    }
}

fn move_bullet(
    mut q: Query<(&mut Transform, Entity), With<Bullet>>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (mut t, e) in q.iter_mut() {
        t.translation.z += BULLET_SPEED * time.delta_seconds();
        if t.translation.z > PLAYGROUND_HEIGHT {
            commands.entity(e).despawn();
        }
    }
}

fn check_collision(
    q: Query<(&Transform, Entity), With<Bullet>>,
    mut e: Query<(&Transform, Entity), With<Enemy>>,
    mut p: Query<(&Transform, &mut Player)>,
    mut commands: Commands
) {
    for (tb, be) in q.iter() {
        // NOTE: Change size if size of mesh changes!
        let aabbb = Aabb2d::new(tb.translation.xz(), Vec2 {x: 0.05, y: 0.15});

        for (te, e) in e.iter_mut() {
            // NOTE: Change size if size of mesh changes!
            let aabbe = Aabb2d::new(te.translation.xz(), Vec2 {x: 0.25, y: 0.25});
            if aabbe.intersects(&aabbb) {
                commands.entity(e).despawn();
                commands.entity(be).despawn();
            }
        }

        let (tp, mut pp) = p.single_mut();
        // NOTE: Change size if size of mesh changes!
        let aabbp = Aabb2d::new(tp.translation.xz(), Vec2 {x: 0.5, y: 0.5});
        if aabbp.intersects(&aabbb) {
            commands.entity(be).despawn();
            pp.health -= 1;
        }
    }

}