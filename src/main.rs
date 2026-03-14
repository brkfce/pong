use bevy::prelude::*;
use std::num;

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct AIPaddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity {
    speed: f32,
    direction: Vec2,
}

#[derive(Component)]
struct Size {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player_paddle, move_ball))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn camera
    commands.spawn(Camera2d);
    // spawn player paddle
    commands.spawn((
        Text2d::new("@\n@"),
        TextFont {
            font_size: 12.8,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::from_array([-500.0, 0.0, 0.0])),
        PlayerPaddle,
        Velocity {
            speed: 0.0,
            direction: Vec2::from_array([0.0, 0.0]),
        },
    ));
    // spawn ball
    commands.spawn((
        Text2d::new("@"),
        TextFont {
            font_size: 12.8,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Ball,
        Velocity {
            speed: 100.0,
            direction: Vec2::from_array([-1.0, 0.0]),
        },
    ));
}

fn move_player_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    q: Single<(&mut Transform, &mut Velocity), With<PlayerPaddle>>,
) {
    let (mut player_transform, mut player_velocity) = q.into_inner();
    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 300.0; // pixels per second
        let delta = direction.normalize() * speed * time.delta_secs();
        player_transform.translation.y += delta.y;
        player_velocity.speed = speed;
        player_velocity.direction = direction;
    }
}

fn move_ball(
    time: Res<Time>,
    mut q: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    paddles: Query<(&Transform, &Velocity), Without<Ball>>,
) {
    let (mut ball_transform, mut ball_velocity) = q.into_inner();
    let delta = ball_velocity.direction.normalize() * ball_velocity.speed * time.delta_secs();
    ball_transform.translation.y += delta.y;
    ball_transform.translation.x += delta.x;

    for paddle in &paddles {
        let paddle_transform = paddle.0;
        let paddle_velocity = paddle.1;
        if ball_transform.translation.x.abs() >= paddle_transform.translation.x.abs() {
            ball_velocity.direction.x = ball_velocity.direction.x * -1.0;
        }
    }
}
