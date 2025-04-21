use bevy::{asset::AssetMetaCheck, prelude::*};
use shining_piano_core::ShiningPianoPlugin;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen]
pub fn run_app() {
    EXIT_REQUESTED.store(false, Ordering::SeqCst);
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#app-canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            ShiningPianoPlugin,
        ))
        .add_systems(Update, check_game_exit)
        .run();
}

#[wasm_bindgen]
pub fn stop_app() {
    EXIT_REQUESTED.store(true, Ordering::SeqCst);
}

fn check_game_exit(
    mut exit: EventWriter<AppExit>
) {
    if EXIT_REQUESTED.load(Ordering::SeqCst) {
        exit.send(AppExit::Success);
    }
}

