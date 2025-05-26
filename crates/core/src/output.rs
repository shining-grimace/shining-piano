use crate::{KeyEvent, StartProgramEvent};
use bevy::prelude::*;
use bevy_midi_graph::{
    GraphAssetLoader, LoopFileSource, MidiFileSource, MidiGraphAudioContext, OneShotFileSource,
    Sf2FileSource,
    config::{Config, SoundSource},
};

const PROGRAM_NO: usize = 0;

pub struct OutputPlugin;

impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_audio)
            .add_systems(Update, play_key_events)
            .add_systems(PostUpdate, change_program);
    }
}

fn configure_audio(
    mut audio_context: ResMut<MidiGraphAudioContext>,
    server: Res<AssetServer>,
    midi_assets: Res<Assets<MidiFileSource>>,
    sf2_assets: Res<Assets<Sf2FileSource>>,
    loop_assets: Res<Assets<LoopFileSource>>,
    one_shot_assets: Res<Assets<OneShotFileSource>>,
) {
    let config = Config {
        root: SoundSource::stock_square_wave(),
    };
    let loader = GraphAssetLoader::new(
        &server,
        &midi_assets,
        &sf2_assets,
        &loop_assets,
        &one_shot_assets,
    );
    audio_context
        .store_new_program(PROGRAM_NO, &config, &loader)
        .unwrap();
    audio_context.change_program(PROGRAM_NO).unwrap();
}

fn play_key_events(
    mut events: EventReader<KeyEvent>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
) -> Result<(), BevyError> {
    if events.is_empty() {
        return Ok(());
    }
    for event in events.read() {
        let event_channel = audio_context.get_event_sender();
        event_channel.send(event.message.clone())?;
    }
    Ok(())
}

fn change_program(
    mut events: EventReader<StartProgramEvent>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
) {
    for event in events.read() {
        audio_context.change_program(event.program_no).unwrap();
        println!("DID CHANGE PROGRAM: {}", event.program_no);
    }
}
