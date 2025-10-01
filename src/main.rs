use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;
use crate::game::Pong;

mod game;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                ..default()
            }),
            Wireframe2dPlugin::default()
        ))
        .add_plugins(Pong)
        .run();
}
