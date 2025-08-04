use crate::StartProgramEvent;
use bevy::{asset::LoadState, prelude::*};
use bevy_midi_graph::{
    GraphAssetLoader, MidiFileSource, MidiGraph, MidiGraphAudioContext, Sf2FileSource,
    WaveFileSource,
};

const DEFAULT_PROGRAM: usize = 1;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProgramAssets>()
            .add_systems(Startup, init_program_assets)
            .add_systems(Update, check_graph_assets_ready);
    }
}

#[derive(Resource, Default)]
pub struct ProgramAssets {
    pub programs: Vec<(LoadState, usize, Handle<MidiGraph>)>,
}

fn init_program_assets(server: Res<AssetServer>, mut program_data: ResMut<ProgramAssets>) {
    let asset_names = [
        "f1.json", "f2.json", "f3.json", "f4.json", "f5.json", "f6.json", "f7.json", "f8.json",
        "f9.json", "f10.json", "f11.json", "f12.json",
    ];
    for (index, name) in asset_names.iter().enumerate() {
        program_data
            .programs
            .push((LoadState::NotLoaded, index + 1, server.load(*name)));
    }
}

fn check_graph_assets_ready(
    server: Res<AssetServer>,
    graph_assets: Res<Assets<MidiGraph>>,
    midi_assets: Res<Assets<MidiFileSource>>,
    sf2_assets: Res<Assets<Sf2FileSource>>,
    wav_assets: Res<Assets<WaveFileSource>>,
    mut program_data: ResMut<ProgramAssets>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
    mut events: EventWriter<StartProgramEvent>,
    mut completed: Local<bool>,
) {
    if *completed {
        return;
    }
    for asset in program_data.programs.iter_mut() {
        match asset.0 {
            LoadState::Loaded | LoadState::Failed(_) => continue,
            _ => {}
        };
        let updated_load_state = server.load_state(asset.2.id());
        match updated_load_state {
            LoadState::Failed(_) | LoadState::NotLoaded => {
                asset.0 = updated_load_state;
                continue;
            }
            LoadState::Loaded | LoadState::Loading => {
                let is_loaded = server.is_loaded_with_dependencies(asset.2.id());
                if !is_loaded {
                    return;
                }
                asset.0 = LoadState::Loaded;

                let mut loader =
                    GraphAssetLoader::new(&server, &midi_assets, &sf2_assets, &wav_assets);
                let graph = graph_assets.get(&asset.2).unwrap();
                audio_context
                    .store_new_program(asset.1, &graph.config, &mut loader)
                    .unwrap();
                println!("DID STORE PROGRAM: {}", asset.1);
            }
        }
    }

    *completed = true;
    events.write(StartProgramEvent {
        program_no: DEFAULT_PROGRAM,
    });
}
