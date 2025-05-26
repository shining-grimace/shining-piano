use bevy::prelude::*;
use bevy_midi_graph::{MidiGraphPlugin, midi::Message};

mod assets;
mod graphics;
mod input;
mod output;
mod utils;

#[derive(Event, Deref, DerefMut, Debug)]
pub struct StartProgramEvent {
    pub program_no: usize,
}

#[derive(Event, Deref, DerefMut, Debug)]
pub struct KeyEvent {
    pub register: KeyboardRegister,
    #[deref]
    pub message: Message,
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyboardRegister {
    Lower,
    Upper,
}

#[derive(Resource)]
pub struct Settings {
    pub note_on_z: u8,
    pub note_on_q: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            note_on_z: 36,
            note_on_q: 48,
        }
    }
}

pub struct ShiningPianoPlugin;

impl Plugin for ShiningPianoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartProgramEvent>()
            .add_event::<KeyEvent>()
            .insert_resource(Settings::default())
            .add_plugins((
                MidiGraphPlugin,
                graphics::GraphicsPlugin,
                input::InputPlugin,
                output::OutputPlugin,
                assets::AssetsPlugin,
            ));
    }
}
