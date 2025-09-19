use crate::resources::*;
use crate::systems::*;
use bevy::prelude::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(GameConfig::default())
            .insert_resource(GameState::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_paddle_input,
                    move_ball,
                    handle_ball_paddle_collision,
                    handle_ball_wall_collision,
                    print_ball_properties,
                    print_player_properties,
                ),
            );
    }
}
