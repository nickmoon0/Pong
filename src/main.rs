use bevy::prelude::*;
use crate::game::Pong;

mod game;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                ..default()
            }),
        ))
        .add_plugins(Pong)
        .run();
}
