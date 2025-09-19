use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;
use pong::PongPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..Default::default()
            }),
            Wireframe2dPlugin::default(),
        ))
        .add_plugins(PongPlugin)
        .run();
}
