use bevy::prelude::*;
use shining_piano_core::ShiningPianoPlugin;

pub fn run_app() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ShiningPianoPlugin
        ))
        .run();
}
