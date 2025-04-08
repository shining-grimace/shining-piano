use crate::KeyboardRegister;

pub fn note_is_black(note: u8) -> bool {
    match note % 12 {
        1 | 3 | 6 | 8 | 10 => true,
        _ => false
    }
}

pub fn on_lower(lower: bool, note: Option<u8>) -> Option<(KeyboardRegister, u8)> {
    note.map(|n| {
        (
            match lower {
                true => KeyboardRegister::Lower,
                false => KeyboardRegister::Upper,
            },
            n,
        )
    })
}

pub fn make_note(from_note: u8, white_key_advance: usize, black_key: bool) -> Option<u8> {
    match black_key {
        false => Some(add_white_keys(from_note, white_key_advance)),
        true => black_key_above(add_white_keys(from_note, white_key_advance)),
    }
}

fn add_white_keys(from_note: u8, number_of_keys: usize) -> u8 {
    let mut arrived_note = from_note;
    for _ in 0..number_of_keys {
        arrived_note += match arrived_note % 12 {
            0 | 2 | 5 | 7 | 9 => 2,
            4 | 11 => 1,
            _ => panic!("INTERNAL: Unexpectedly ascending from a black key"),
        };
    }
    arrived_note
}

fn black_key_above(from_note: u8) -> Option<u8> {
    match from_note % 12 {
        0 | 2 | 5 | 7 | 9 => Some(from_note + 1),
        1 | 3 | 6 | 8 | 10 => panic!("INTERNAL: Unexpectedly looking above a black key"),
        _ => None,
    }
}
