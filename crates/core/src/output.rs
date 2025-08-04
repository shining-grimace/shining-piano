use crate::{KeyEvent, StartProgramEvent};
use bevy::prelude::*;
use bevy_midi_graph::{
    GraphAssetLoader, MidiFileSource, MidiGraphAudioContext, Sf2FileSource, WaveFileSource,
    midi::{
        event::Balance,
        node::{NodeConfigData, SquareWave},
    },
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
    wav_assets: Res<Assets<WaveFileSource>>,
) {
    let config = NodeConfigData(Box::new(SquareWave {
        node_id: None,
        balance: Balance::Both,
        amplitude: 0.125,
        duty_cycle: 0.25,
    }));
    let mut loader = GraphAssetLoader::new(&server, &midi_assets, &sf2_assets, &wav_assets);
    audio_context
        .store_new_program(PROGRAM_NO, &config, &mut loader)
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
