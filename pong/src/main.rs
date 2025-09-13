use bevy::prelude::*;

#[derive(Component)]
struct Player {
    side: Side,
}

#[derive(Component, Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    direction: Vec2,
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((Player { side: Side::Left },));
    commands.spawn((Player { side: Side::Right },));
    commands.spawn((Ball {
        velocity: Vec2::new(100.0, 100.0),
        direction: Vec2::new(1.0, 1.0),
        x: 0.0,
        y: 0.0,
    },));
}

fn print_ball_properties(query: Query<&Ball>) {
    for ball in &query {
        println!("Ball position: ({}, {})", ball.x, ball.y);
        println!("Ball velocity: ({}, {})", ball.velocity.x, ball.velocity.y);
        println!("Ball direction: ({}, {})", ball.direction.x, ball.direction.y);
    }
}

fn print_player_properties(query: Query<&Player>) {
    for player in &query {
        println!("Player side: {:?}", player.side);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (print_ball_properties, print_player_properties))
        .run();
}
