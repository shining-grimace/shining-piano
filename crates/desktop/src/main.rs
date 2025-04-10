use bevy::prelude::*;
use shining_piano_core::ShiningPianoPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".to_owned(),
                ..default()
            }),
            ShiningPianoPlugin,
        ))
        .run();
}
