use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
) {
    commands.spawn(Camera2d);

    commands.spawn((Player { side: Side::Left },));
    commands.spawn((Player { side: Side::Right },));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(20.0, 100.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(windows.single().unwrap().width() / 2.0 - 10., 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(20.0, 100.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(-windows.single().unwrap().width() / 2.0 + 10., 0.0, 0.0),
    ));

    commands.spawn((Ball {
        velocity: Vec2::new(100.0, 100.0),
        direction: Vec2::new(1.0, 1.0),
        x: 0.0,
        y: 0.0,
    },));
}

fn print_ball_properties(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Ball>) {
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

fn print_player_properties(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Player>) {
    if timer.0.tick(time.delta()).just_finished() {
        for player in &query {
            println!("Player side: {:?}", player.side);
        }
    }
}

#[derive(Resource)]
struct PrintTimer(Timer);

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, (print_ball_properties, print_player_properties));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default()))
        .add_plugins(PongPlugin)
        .run();
}
