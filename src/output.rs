use crate::{KeyEvent, KeyboardRegister};
use bevy::prelude::*;
use bevy_midi_graph::{
    config::{Config, FontSource, RangeSource, SoundSource},
    MidiGraphAudioContext,
};

const NODE_ID_LOWER: u64 = 0;
const NODE_ID_UPPER: u64 = 1;

pub struct OutputPlugin;

impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_audio)
            .add_systems(Update, play_key_events);
    }
}

fn configure_audio(mut audio_context: ResMut<MidiGraphAudioContext>) {
    let config = Config {
        root: SoundSource::Combiner {
            node_id: None,
            sources: vec![
                SoundSource::event_receiver(
                    Some(NODE_ID_LOWER),
                    SoundSource::Font {
                        node_id: None,
                        config: FontSource::Ranges(vec![RangeSource {
                            source: SoundSource::stock_envelope(SoundSource::SquareWave {
                                node_id: None,
                                amplitude: 0.125,
                                duty_cycle: 0.25,
                            }),
                            lower: 0,
                            upper: 127,
                        }]),
                    },
                ),
                SoundSource::event_receiver(
                    Some(NODE_ID_UPPER),
                    SoundSource::Font {
                        node_id: None,
                        config: FontSource::Ranges(vec![RangeSource {
                            source: SoundSource::stock_triangle_wave(),
                            lower: 0,
                            upper: 127,
                        }]),
                    },
                ),
            ],
        },
    };
    audio_context.swap_graph(&config).unwrap();
}

fn play_key_events(
    mut events: EventReader<KeyEvent>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
) {
    if events.is_empty() {
        return;
    }
    for event in events.read() {
        let node_id = match event.register {
            KeyboardRegister::Lower => NODE_ID_LOWER,
            KeyboardRegister::Upper => NODE_ID_UPPER,
        };
        let event_channel = audio_context
            .event_channel(node_id)
            .expect("No event channel found");
        event_channel
            .send(event.event.clone())
            .expect("INTERNAL: Send failure");
    }
}
