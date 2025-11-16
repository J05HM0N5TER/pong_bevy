#![allow(clippy::unnecessary_cast)]

use avian2d::math::{AdjustPrecision, Scalar, Vector};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels.
            // The unit allows the engine to tune its parameters for the scale of the world, improving stability.
            PhysicsPlugins::default().with_length_unit(20.0),
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
struct Marble;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2d);

    let square_sprite = Sprite {
        color: Color::srgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Player
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, 0., 0.0).with_scale(Vec3::new(1.0, 5.0, 1.0)),
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 50.0),
        Player,
    ));

    // Ceiling
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, 50.0 * 6.0, 0.0).with_scale(Vec3::new(20.0, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Floor
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, -50.0 * 6.0, 0.0).with_scale(Vec3::new(20.0, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Left wall
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(-50.0 * 9.5, 0.0, 0.0).with_scale(Vec3::new(1.0, 11.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        square_sprite,
        Transform::from_xyz(50.0 * 9.5, 0.0, 0.0).with_scale(Vec3::new(1.0, 11.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));

    let marble_radius = 10.0;
    let marble_mesh = meshes.add(Circle::new(marble_radius));
    let marble_material = materials.add(Color::srgb(0.2, 0.7, 0.9));

    commands.spawn((
        Mesh2d(marble_mesh),
        MeshMaterial2d(marble_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Collider::circle(marble_radius as Scalar),
        Marble,
        Restitution::new(1.).with_combine_rule(CoefficientCombine::Max),
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
    ));
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut LinearVelocity, With<Player>>,
) {
    const MOVEMENT_SPEED: f32 = 10000.;
    let mut moving = false;
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    let mut player_movement = Vec2::splat(0.);

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player_movement.x -= 1.;
        moving = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player_movement.x += 1.;
        moving = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        player_movement.y += 1.;
        moving = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        player_movement.y -= 1.;
        moving = true;
    }
    player.0 = {
        if moving {
            player_movement.normalize() * MOVEMENT_SPEED * delta_time
        } else {
            Vec2::ZERO
        }
    }
}
