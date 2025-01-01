use crate::{
    utils::{make_note, on_lower},
    KeyEvent, KeyboardRegister, Settings,
};
use bevy::prelude::*;
use bevy_midi_graph::midi::{NodeEvent, NoteEvent};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, post_input_events);
    }
}

fn post_input_events(
    inputs: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut events: EventWriter<KeyEvent>,
    mut quit_signal: EventWriter<AppExit>,
) {
    for key in inputs.get_just_pressed() {
        if let Some((register, note)) = note_from_key_code(key, &settings) {
            events.send(KeyEvent {
                register,
                event: NodeEvent::Note {
                    note,
                    event: NoteEvent::NoteOn { vel: 1.0 },
                },
            });
        }
    }
    for key in inputs.get_just_released() {
        if let Some((register, note)) = note_from_key_code(key, &settings) {
            events.send(KeyEvent {
                register,
                event: NodeEvent::Note {
                    note,
                    event: NoteEvent::NoteOff { vel: 1.0 },
                },
            });
        }
    }
    if inputs.just_pressed(KeyCode::Escape) {
        quit_signal.send(AppExit::Success);
    }
}

fn note_from_key_code(key: &KeyCode, settings: &Settings) -> Option<(KeyboardRegister, u8)> {
    let note_on_z = settings.note_on_z;
    match key {
        KeyCode::KeyZ => on_lower(true, make_note(note_on_z, 0, false)),
        KeyCode::KeyS => on_lower(true, make_note(note_on_z, 0, true)),
        KeyCode::KeyX => on_lower(true, make_note(note_on_z, 1, false)),
        KeyCode::KeyD => on_lower(true, make_note(note_on_z, 1, true)),
        KeyCode::KeyC => on_lower(true, make_note(note_on_z, 2, false)),
        KeyCode::KeyF => on_lower(true, make_note(note_on_z, 2, true)),
        KeyCode::KeyV => on_lower(true, make_note(note_on_z, 3, false)),
        KeyCode::KeyG => on_lower(true, make_note(note_on_z, 3, true)),
        KeyCode::KeyB => on_lower(true, make_note(note_on_z, 4, false)),
        KeyCode::KeyH => on_lower(true, make_note(note_on_z, 4, true)),
        KeyCode::KeyN => on_lower(true, make_note(note_on_z, 5, false)),
        KeyCode::KeyJ => on_lower(true, make_note(note_on_z, 5, true)),
        KeyCode::KeyM => on_lower(true, make_note(note_on_z, 6, false)),
        KeyCode::KeyK => on_lower(true, make_note(note_on_z, 6, true)),
        KeyCode::Comma => on_lower(true, make_note(note_on_z, 7, false)),
        KeyCode::KeyL => on_lower(true, make_note(note_on_z, 7, true)),
        KeyCode::Period => on_lower(true, make_note(note_on_z, 8, false)),
        KeyCode::Semicolon => on_lower(true, make_note(note_on_z, 8, true)),
        KeyCode::Slash => on_lower(true, make_note(note_on_z, 9, false)),
        KeyCode::Quote => on_lower(true, make_note(note_on_z, 9, true)),
        KeyCode::KeyQ => on_lower(false, make_note(note_on_z, 7, false)),
        KeyCode::Digit2 => on_lower(false, make_note(note_on_z, 7, true)),
        KeyCode::KeyW => on_lower(false, make_note(note_on_z, 8, false)),
        KeyCode::Digit3 => on_lower(false, make_note(note_on_z, 8, true)),
        KeyCode::KeyE => on_lower(false, make_note(note_on_z, 9, false)),
        KeyCode::Digit4 => on_lower(false, make_note(note_on_z, 9, true)),
        KeyCode::KeyR => on_lower(false, make_note(note_on_z, 10, false)),
        KeyCode::Digit5 => on_lower(false, make_note(note_on_z, 10, true)),
        KeyCode::KeyT => on_lower(false, make_note(note_on_z, 11, false)),
        KeyCode::Digit6 => on_lower(false, make_note(note_on_z, 11, true)),
        KeyCode::KeyY => on_lower(false, make_note(note_on_z, 12, false)),
        KeyCode::Digit7 => on_lower(false, make_note(note_on_z, 12, true)),
        KeyCode::KeyU => on_lower(false, make_note(note_on_z, 13, false)),
        KeyCode::Digit8 => on_lower(false, make_note(note_on_z, 13, true)),
        KeyCode::KeyI => on_lower(false, make_note(note_on_z, 14, false)),
        KeyCode::Digit9 => on_lower(false, make_note(note_on_z, 14, true)),
        KeyCode::KeyO => on_lower(false, make_note(note_on_z, 15, false)),
        KeyCode::Digit0 => on_lower(false, make_note(note_on_z, 15, true)),
        KeyCode::KeyP => on_lower(false, make_note(note_on_z, 16, false)),
        KeyCode::Minus => on_lower(false, make_note(note_on_z, 16, true)),
        KeyCode::BracketLeft => on_lower(false, make_note(note_on_z, 16, false)),
        KeyCode::Equal => on_lower(false, make_note(note_on_z, 16, true)),
        KeyCode::BracketRight => on_lower(false, make_note(note_on_z, 16, false)),
        _ => None,
    }
}
