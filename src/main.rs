use avian2d::math::{AdjustPrecision, Scalar, Vector};
use avian2d::prelude::*;
use bevy::color::palettes::css;
use bevy::prelude::*;

const MARBLE_RADIUS: Scalar = 10.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels.
            // The unit allows the engine to tune its parameters for the scale of the world, improving stability.
            PhysicsPlugins::default().with_length_unit(10.0),
        ))
        .insert_resource(ClearColor(Color::from(css::BLACK)))
        // Disable gravity
        .insert_resource(Gravity(Vector::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(MARBLE_RADIUS),
    Restitution::new(1.0),
    Friction::new(0.0),
    MaxLinearSpeed(200.0)
)]
struct Ball;

#[derive(Component)]
#[require(RigidBody::Kinematic)]
struct Player;

#[derive(Component)]
#[require(RigidBody::Static, Collider::rectangle(50.0, 50.0))]
struct Gutter;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2d);

    let square_sprite = Sprite {
        color: Color::from(css::WHITE),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Player
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(-300.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 4.0, 1.0)),
        Collider::rectangle(50.0, 50.0),
        Player,
    ));

    let floor_scale = Vec3::new(20.0, 1.0, 1.0);
    // Ceiling
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, 50.0 * 6.0, 0.0).with_scale(floor_scale),
        Gutter,
    ));
    // Floor
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, -50.0 * 6.0, 0.0).with_scale(floor_scale),
        Gutter,
    ));
    let wall_scale = Vec3::new(1.0, 11.0, 1.0);
    // Left wall
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(-50.0 * 9.5, 0.0, 0.0).with_scale(wall_scale),
        Gutter,
    ));
    // Right wall
    commands.spawn((
        square_sprite,
        Transform::from_xyz(50.0 * 9.5, 0.0, 0.0).with_scale(wall_scale),
        Gutter,
    ));

    let marble_mesh = meshes.add(Circle::new(MARBLE_RADIUS));
    let marble_material = materials.add(Color::from(css::WHITE));

    commands.spawn((
        Mesh2d(marble_mesh),
        MeshMaterial2d(marble_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball,
        // Max bounce
        Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
        // Min friction
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        // Set default speed
        LinearVelocity(Vec2::new(500.0, 500.0)),
    ));
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut LinearVelocity, &Position), With<Player>>,
) {
    const MOVEMENT_SPEED: f32 = 10000.0;
    let (mut velocity, position) = player.into_inner();
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    let mut player_movement = Vec2::splat(0.0);

    // if keyboard_input.pressed(KeyCode::ArrowLeft) {
    //     player_movement.x -= 1.0;
    // }
    // if keyboard_input.pressed(KeyCode::ArrowRight) {
    //     player_movement.x += 1.0;
    // }
    let play_space_height: Scalar = 200.0;
    if keyboard_input.pressed(KeyCode::ArrowUp) && position.y <= play_space_height {
        player_movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) && position.y >= -play_space_height {
        player_movement.y -= 1.0;
    }
    velocity.0 = {
        if player_movement == Vec2::ZERO {
            Vec2::ZERO
        } else {
            player_movement.normalize() * MOVEMENT_SPEED * delta_time
        }
    }
}
