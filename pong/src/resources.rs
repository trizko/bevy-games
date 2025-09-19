use bevy::prelude::*;

#[derive(Resource)]
pub struct PrintTimer(pub Timer);

#[derive(Resource)]
pub struct GameConfig {
    pub paddle_speed: f32,
    pub ball_speed: f32,
    pub paddle_height: f32,
    pub paddle_width: f32,
    pub ball_size: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            paddle_speed: 300.0,
            ball_speed: 400.0,
            paddle_height: 100.0,
            paddle_width: 20.0,
            ball_size: 20.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    pub score_left: u32,
    pub score_right: u32,
    pub game_running: bool,
}
