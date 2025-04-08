use bevy::prelude::*;
use shining_piano_core::ShiningPianoPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ShiningPianoPlugin
        ))
        .run();
}
