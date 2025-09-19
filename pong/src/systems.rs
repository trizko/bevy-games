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
        },
        Mesh2d(meshes.add(Rectangle::new(config.ball_size, config.ball_size))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&Ball, &mut Transform)>,
) {
    for (ball, mut transform) in &mut ball_query {
        // Move ball based on velocity and time
        let movement = ball.velocity * time.delta_secs();
        let _old_x = transform.translation.x;
        let _old_y = transform.translation.y;
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

pub fn handle_ball_wall_collision(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let half_height = window.height() / 2.0;
    let half_width = window.width() / 2.0;

    for (mut ball, mut transform) in &mut ball_query {
        let mut velocity = ball.velocity;

        // Check top and bottom wall collisions
        if transform.translation.y >= half_height || transform.translation.y <= -half_height {
            velocity.y = -velocity.y; // Reverse Y direction
        }

        // Check if ball went out of bounds (scoring)
        if transform.translation.x >= half_width {
            // Right player scored
            println!("Right player scored!");
            // Reset ball to center
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            velocity.x = -velocity.x; // Reverse X direction
        } else if transform.translation.x <= -half_width {
            // Left player scored
            println!("Left player scored!");
            // Reset ball to center
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            velocity.x = -velocity.x; // Reverse X direction
        }

        ball.velocity = velocity;
    }
}


pub fn print_ball_properties(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<(&Ball, &Transform)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (ball, transform) in &query {
            println!("Ball position: ({}, {})", transform.translation.x, transform.translation.y);
            println!("Ball velocity: ({}, {})", ball.velocity.x, ball.velocity.y);
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
