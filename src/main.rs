use bevy::prelude::*;
use bevy_midi_graph::{midi::NodeEvent, MidiGraphPlugin};

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
    pub event: NodeEvent,
}

#[derive(Debug)]
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

fn main() {
    App::new()
        .add_event::<StartProgramEvent>()
        .add_event::<KeyEvent>()
        .insert_resource(Settings::default())
        .add_plugins((
            DefaultPlugins,
            MidiGraphPlugin,
            graphics::GraphicsPlugin,
            input::InputPlugin,
            output::OutputPlugin,
            assets::AssetsPlugin,
        ))
        .run();
}
