use bevy::prelude::*;
use rand::RngExt;
use std::f32::consts::FRAC_PI_2;

const BOUNCE_SPEED_INCREASE: f32 = 10.0;
const PADDLE_SPEED: f32 = 300.0;

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct AIPaddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct GameBoundary;

#[derive(Component)]
struct Velocity {
    speed: f32,
    direction: Vec2,
}

#[derive(Component, Clone)]
struct Size {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

#[derive(Component)]
struct Score {
    player: u8,
    ai: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player_paddle,
                move_ball,
                paddle_collisions,
                ball_boundary_collisions,
                scoring,
                ai_paddle,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn camera
    commands.spawn(Camera2d);
    // spawn game boundary
    commands.spawn((
        GameBoundary,
        Size {
            top: 350.0,
            bottom: 350.0,
            left: 700.0,
            right: 700.0,
        },
    ));
    // spawn score
    commands.spawn(Score { player: 0, ai: 0 });
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
    q: Single<(&mut Transform, &mut Velocity, &Size), With<PlayerPaddle>>,
    boundary: Single<Entity, With<GameBoundary>>,
    boundary_query: Query<&Size>,
) {
    let (mut player_transform, mut player_velocity, paddle_size) = q.into_inner();
    let mut direction = Vec2::ZERO;

    let boundary_entity = boundary.entity();
    let boundary_size = boundary_query.get(boundary_entity).unwrap();

    if input.pressed(KeyCode::ArrowUp)
        && (player_transform.translation.y + paddle_size.top <= boundary_size.top)
    {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown)
        && (player_transform.translation.y - paddle_size.bottom >= -1.0 * boundary_size.bottom)
    {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * PADDLE_SPEED * time.delta_secs();
        player_transform.translation.y += delta.y;
        player_velocity.speed = PADDLE_SPEED;
        player_velocity.direction = direction;
    }
}

fn move_ball(time: Res<Time>, q: Single<(&mut Transform, &mut Velocity), With<Ball>>) {
    let (mut ball_transform, ball_velocity) = q.into_inner();
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

        // check if ball + size collides with either paddle + size
        if ((paddle_transform.translation.x < 0.0)
            && (ball_transform.translation.x - ball_size.left)
                <= (paddle_transform.translation.x + paddle_size.right)
            && (ball_transform.translation.y - ball_size.top)
                <= (paddle_transform.translation.y + paddle_size.top)
            && (ball_transform.translation.y + ball_size.top)
                >= (paddle_transform.translation.y - paddle_size.bottom))
            || ((paddle_transform.translation.x > 0.0)
                && ((ball_transform.translation.x + ball_size.right)
                    >= (paddle_transform.translation.x - paddle_size.left)
                    && (ball_transform.translation.y - ball_size.top)
                        <= (paddle_transform.translation.y + paddle_size.top)
                    && (ball_transform.translation.y + ball_size.top)
                        >= (paddle_transform.translation.y - paddle_size.bottom)))
        {
            ball_velocity.direction.x *= -1.0;
            ball_velocity.speed += BOUNCE_SPEED_INCREASE;
            let mut rng = rand::rng();
            let rand_angle = rng.random_range(-FRAC_PI_2..FRAC_PI_2) / 5.0;
            ball_velocity.direction = ball_velocity
                .direction
                .rotate(Vec2::new(rand_angle.cos(), rand_angle.sin()));
        }
    }
}

fn ball_boundary_collisions(
    ball_query: Single<(&Transform, &mut Velocity, &Size), With<Ball>>,
    boundary: Single<Entity, With<GameBoundary>>,
    boundary_query: Query<&Size>,
) {
    let (ball_transform, mut ball_velocity, ball_size) = ball_query.into_inner();
    let boundary_entity = boundary.entity();
    let boundary_size = boundary_query.get(boundary_entity).unwrap();

    // check if ball collides with boundary and if so, bounce
    if ((ball_transform.translation.y + ball_size.top) >= boundary_size.top)
        || (ball_transform.translation.y - ball_size.bottom) <= -1.0 * boundary_size.bottom
    {
        ball_velocity.direction.y *= -1.0;
        ball_velocity.speed += BOUNCE_SPEED_INCREASE;
        let mut rng = rand::rng();
        let rand_angle = rng.random_range(-FRAC_PI_2..FRAC_PI_2) / 5.0;
        ball_velocity.direction = ball_velocity
            .direction
            .rotate(Vec2::new(rand_angle.cos(), rand_angle.sin()));
    }
}

fn scoring(
    ball_query: Single<(&mut Transform, &Size), With<Ball>>,
    boundary: Single<Entity, With<GameBoundary>>,
    boundary_query: Query<&Size>,
    mut score: Single<&mut Score, With<Score>>,
) {
    let (mut ball_transform, ball_size) = ball_query.into_inner();
    let boundary_entity = boundary.entity();
    let boundary_size = boundary_query.get(boundary_entity).unwrap();

    // check if ball collides with boundary and if so, score
    if ((ball_transform.translation.x + ball_size.right) >= boundary_size.right)
        || (ball_transform.translation.x - ball_size.left) <= -1.0 * boundary_size.left
    {
        // score for player/ai
        if ball_transform.translation.x > 0.0 {
            score.player += 1;
        } else if ball_transform.translation.x < 0.0 {
            score.ai += 1;
        }
        // reset ball
        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = 0.0;
    }
}

fn ai_paddle(
    /*
    ball_query: Single<&Transform, With<Ball>>,
    ai: Single<Entity, With<AIPaddle>>,
    mut ai_query: Query<(&mut Transform, &Size), Without<GameBoundary>>,
    boundary: Single<Entity, With<GameBoundary>>,
    boundary_query: Query<&Size, Without<AIPaddle>>,
    */
    time: Res<Time>,

    mut set: ParamSet<(
        Single<&Transform, With<Ball>>,
        Single<(&mut Transform, &Size), With<AIPaddle>>,
        Single<&Size, With<GameBoundary>>,
    )>,
) {
    /*
        let boundary_entity = boundary.entity();
        let boundary_size = boundary_query.get(boundary_entity).unwrap();

        let ball_transform = ball_query.into_inner();

        let ai_entity = ai.entity();
        let (mut ai_transform, ai_size) = ai_query.get_mut(ai_entity).unwrap();
    */
    let ball_transform = set.p0().into_inner().clone();
    let boundary_size = set.p2().into_inner().clone();
    let (mut ai_transform, ai_size) = set.p1().into_inner();

    if ball_transform.translation.y > ai_transform.translation.y
        && ai_transform.translation.y + ai_size.top < boundary_size.top
    {
        ai_transform.translation.y += PADDLE_SPEED * time.delta_secs();
    } else if ball_transform.translation.y < ai_transform.translation.y
        && ai_transform.translation.y - ai_size.bottom > -1.0 * boundary_size.bottom
    {
        ai_transform.translation.y -= PADDLE_SPEED * time.delta_secs();
    }
}
