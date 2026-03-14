use bevy::prelude::*;
use rand::RngExt;
use std::f32::consts::FRAC_PI_2;

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
        .add_systems(Update, (move_player_paddle, move_ball, paddle_collisions))
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
        Size {
            top: 75.0,
            bottom: 75.0,
            left: 1.0,
            right: 1.0,
        },
    ));
    // spawn AI paddle
    commands.spawn((
        Text2d::new("@\n@"),
        TextFont {
            font_size: 12.8,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::from_array([500.0, 0.0, 0.0])),
        AIPaddle,
        Velocity {
            speed: 0.0,
            direction: Vec2::from_array([0.0, 0.0]),
        },
        Size {
            top: 75.0,
            bottom: 75.0,
            left: 1.0,
            right: 1.0,
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
        Size {
            top: 10.0,
            bottom: 10.0,
            left: 10.0,
            right: 10.0,
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

fn move_ball(time: Res<Time>, mut q: Single<(&mut Transform, &mut Velocity), With<Ball>>) {
    let (mut ball_transform, mut ball_velocity) = q.into_inner();
    let delta = ball_velocity.direction.normalize() * ball_velocity.speed * time.delta_secs();
    ball_transform.translation.y += delta.y;
    ball_transform.translation.x += delta.x;
}

fn paddle_collisions(
    q: Single<(&Transform, &mut Velocity, &Size), With<Ball>>,
    paddles: Query<(&Transform, &Velocity, &Size), Without<Ball>>,
) {
    let (ball_transform, mut ball_velocity, ball_size) = q.into_inner();

    for paddle in &paddles {
        let paddle_transform = paddle.0;
        let paddle_velocity = paddle.1;
        let paddle_size = paddle.2;

        // check player paddle
        if paddle_transform.translation.x < 0.0 {
            if (ball_transform.translation.x - ball_size.left)
                <= (paddle_transform.translation.x + paddle_size.right)
                && (ball_transform.translation.y - ball_size.top)
                    <= (paddle_transform.translation.y + paddle_size.top)
                && (ball_transform.translation.y + ball_size.top)
                    >= (paddle_transform.translation.y - paddle_size.bottom)
            {
                ball_velocity.direction.x = ball_velocity.direction.x * -1.0;
                ball_velocity.speed = ball_velocity.speed + 10.0;
                let mut rng = rand::rng();
                let rand_angle = rng.random_range(-FRAC_PI_2..FRAC_PI_2) / 5.0;
                ball_velocity.direction = ball_velocity
                    .direction
                    .rotate(Vec2::new(rand_angle.cos(), rand_angle.sin()));
            }
        } else if paddle_transform.translation.x > 0.0 {
            if (ball_transform.translation.x + ball_size.right)
                >= (paddle_transform.translation.x - paddle_size.left)
                && (ball_transform.translation.y - ball_size.top)
                    <= (paddle_transform.translation.y + paddle_size.top)
                && (ball_transform.translation.y + ball_size.top)
                    >= (paddle_transform.translation.y - paddle_size.bottom)
            {
                ball_velocity.direction.x = ball_velocity.direction.x * -1.0;
                ball_velocity.speed =
                    ball_velocity.speed + paddle_velocity.speed * ball_velocity.direction.y;
            }
        }
    }
}
