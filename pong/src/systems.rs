use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
    config: Res<GameConfig>,
) {
    commands.spawn(Camera2d);

    // Spawn left player with their paddle
    let _left_player = commands.spawn((
        Player {
            side: Side::Left,
            score: 0,
        },
        Paddle {
            speed: config.paddle_speed,
        },
        Mesh2d(meshes.add(Rectangle::new(config.paddle_width, config.paddle_height))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(-windows.single().unwrap().width() / 2.0 + 10., 0.0, 0.0),
    )).id();

    // Spawn right player with their paddle
    let _right_player = commands.spawn((
        Player {
            side: Side::Right,
            score: 0,
        },
        Paddle {
            speed: config.paddle_speed,
        },
        Mesh2d(meshes.add(Rectangle::new(config.paddle_width, config.paddle_height))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(windows.single().unwrap().width() / 2.0 - 10., 0.0, 0.0),
    )).id();

    // Spawn ball
    commands.spawn((
        Ball {
            velocity: Vec2::new(config.ball_speed, config.ball_speed),
            direction: Vec2::new(1.0, 1.0),
            x: 0.0,
            y: 0.0,
        },
        Mesh2d(meshes.add(Rectangle::new(config.ball_size, config.ball_size))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn print_ball_properties(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Ball>) {
    if timer.0.tick(time.delta()).just_finished() {
        for ball in &query {
            println!("Ball position: ({}, {})", ball.x, ball.y);
            println!("Ball velocity: ({}, {})", ball.velocity.x, ball.velocity.y);
            println!(
                "Ball direction: ({}, {})",
                ball.direction.x, ball.direction.y
            );
        }
    }
}

pub fn print_player_properties(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<(&Player, &Paddle)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (player, paddle) in &query {
            println!("Player {:?}: score={}, paddle_speed={}", player.side, player.score, paddle.speed);
        }
    }
}
