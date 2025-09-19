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

pub fn handle_ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &Transform), Without<Paddle>>,
    paddle_query: Query<(&Player, &Transform), With<Paddle>>,
    config: Res<GameConfig>,
) {
    for (mut ball, ball_transform) in &mut ball_query {
        let ball_pos = ball_transform.translation;
        let ball_radius = config.ball_size / 2.0;
        let paddle_width = config.paddle_width;
        let paddle_height = config.paddle_height;

        for (player, paddle_transform) in &paddle_query {
            let paddle_pos = paddle_transform.translation;
            let paddle_half_width = paddle_width / 2.0;
            let paddle_half_height = paddle_height / 2.0;

            // Check if ball is colliding with paddle
            let ball_left = ball_pos.x - ball_radius;
            let ball_right = ball_pos.x + ball_radius;
            let ball_top = ball_pos.y + ball_radius;
            let ball_bottom = ball_pos.y - ball_radius;

            let paddle_left = paddle_pos.x - paddle_half_width;
            let paddle_right = paddle_pos.x + paddle_half_width;
            let paddle_top = paddle_pos.y + paddle_half_height;
            let paddle_bottom = paddle_pos.y - paddle_half_height;

            // AABB collision detection
            if ball_right >= paddle_left && ball_left <= paddle_right &&
               ball_top >= paddle_bottom && ball_bottom <= paddle_top {
                
                // Ball hit the paddle - reverse X direction
                ball.velocity.x = -ball.velocity.x;
                
                // Add some Y velocity based on where the ball hit the paddle
                let hit_point = (ball_pos.y - paddle_pos.y) / paddle_half_height;
                ball.velocity.y += hit_point * 100.0; // Add some spin
                
                // Ensure minimum speed
                let speed = ball.velocity.length();
                if speed < 200.0 {
                    ball.velocity = ball.velocity.normalize() * 200.0;
                }
                
                println!("Ball hit {:?} paddle!", player.side);
                break; // Only handle one collision per frame
            }
        }
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

pub fn handle_paddle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddle_query: Query<(&Player, &Paddle, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let half_height = window.height() / 2.0;
    let paddle_half_height = 50.0; // Half of paddle height (100.0 / 2)

    for (player, paddle, mut transform) in &mut paddle_query {
        let mut direction = 0.0;

        match player.side {
            Side::Left => {
                // Left player uses W and S keys
                if keyboard_input.pressed(KeyCode::KeyW) {
                    direction += 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    direction -= 1.0;
                }
            }
            Side::Right => {
                // Right player uses Up and Down arrow keys
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    direction += 1.0;
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    direction -= 1.0;
                }
            }
        }

        // Move paddle
        if direction != 0.0 {
            let movement = direction * paddle.speed * time.delta_secs();
            transform.translation.y += movement;

            // Keep paddle within screen bounds
            let max_y = half_height - paddle_half_height;
            let min_y = -half_height + paddle_half_height;
            
            transform.translation.y = transform.translation.y.clamp(min_y, max_y);
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
