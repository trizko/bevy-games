use bevy::prelude::*;
use crate::systems::*;
use crate::resources::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(GameConfig::default())
            .insert_resource(GameState::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (print_ball_properties, print_player_properties));
    }
}