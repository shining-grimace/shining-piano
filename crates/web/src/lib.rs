use bevy::prelude::*;
use shining_piano_core::ShiningPianoPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#app-canvas".into()),
                    ..default()
                }),
                ..default()
            }),
            ShiningPianoPlugin,
        ))
        .run();
}
