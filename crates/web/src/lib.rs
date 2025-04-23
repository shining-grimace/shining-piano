use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*};
use shining_piano_core::ShiningPianoPlugin;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Start running the Bevy app.
/// Plugin configuration is specific to WASM to avoid issues:
/// - WindowPlugin configured to use the existing canvas rather than
///   creating a new one
/// - AssetPlugin configured to not check for meta files that never exist
/// - LogPlugin disabled so it doesn't try to attach to the process
///   multiple times (which ponics)
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
                })
                .build()
                .disable::<LogPlugin>()
            ,
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

