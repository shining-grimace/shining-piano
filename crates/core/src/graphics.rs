use crate::{
    KeyEvent, KeyboardRegister, Settings,
    utils::{make_note, note_is_black},
};
use bevy::prelude::*;
use bevy_midi_graph::midi::event::{Event, Message};

pub struct GraphicsPlugin;

const BASE_HEIGHT: f32 = 0.2;
const BASE_MARGIN: f32 = 0.1;
const KEY_WIDTH: f32 = 0.2;
const KEY_GAP: f32 = 0.02;
const KEY_DEPTH: f32 = 0.8;
const KEY_HEIGHT: f32 = 0.1;
const KEY_BLACK_DEPTH: f32 = 0.5;
const ROW_ELEVATION: f32 = 0.3;
const ROW_OFFSET: f32 = 0.6;
const KEY_DEPRESSION: f32 = 0.05;

#[derive(Component)]
struct KeyWithNote {
    note: u8,
    register: KeyboardRegister,
}

#[derive(Resource, Default)]
struct PianoMaterials {
    pub plastic: Handle<StandardMaterial>,
    pub ivory: Handle<StandardMaterial>,
    pub ebony: Handle<StandardMaterial>,
    pub illuminated: Handle<StandardMaterial>,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            ..default()
        })
        .insert_resource(PianoMaterials::default())
        .add_systems(Startup, create_piano)
        .add_systems(Update, highlight_key_events);
    }
}

fn create_piano(
    mut commands: Commands,
    settings: Res<Settings>,
    mut materials: ResMut<PianoMaterials>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    materials.plastic = material_assets.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.8).into(),
        ..default()
    });
    materials.ivory = material_assets.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.9).into(),
        ..default()
    });
    materials.ebony = material_assets.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2).into(),
        ..default()
    });
    materials.illuminated = material_assets.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.4, 0.4).into(),
        ..default()
    });

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let base_width = 12.0 * KEY_WIDTH + 11.0 * KEY_GAP + 2.0 * BASE_MARGIN;
    let base_depth = KEY_DEPTH + ROW_OFFSET + 2.0 * BASE_MARGIN;
    new_cube(
        255,
        KeyboardRegister::Lower,
        &mut commands,
        materials.plastic.clone(),
        &mut mesh_assets,
        Vec3::new(0.0, -0.5 * BASE_HEIGHT, -0.5 * base_depth),
        Vec3::new(base_width, BASE_HEIGHT, base_depth),
    );

    for i in 0..10 {
        let x =
            -0.5 * base_width + BASE_MARGIN + KEY_WIDTH + (i as f32 + 1.0) * (KEY_WIDTH + KEY_GAP);
        let note = make_note(settings.note_on_z, i, false).expect("Failed getting white note");
        new_cube(
            note,
            KeyboardRegister::Lower,
            &mut commands,
            materials.ivory.clone(),
            &mut mesh_assets,
            Vec3::new(x, 0.5 * KEY_HEIGHT, -BASE_MARGIN - 0.5 * KEY_DEPTH),
            Vec3::new(KEY_WIDTH, KEY_HEIGHT, KEY_DEPTH),
        );
        if let Some(note) = make_note(settings.note_on_z, i, true) {
            new_cube(
                note,
                KeyboardRegister::Lower,
                &mut commands,
                materials.ebony.clone(),
                &mut mesh_assets,
                Vec3::new(
                    x + 0.5 * (KEY_WIDTH + KEY_GAP),
                    1.5 * KEY_HEIGHT,
                    -BASE_MARGIN - KEY_DEPTH + 0.5 * KEY_BLACK_DEPTH,
                ),
                Vec3::new(KEY_WIDTH, KEY_HEIGHT, KEY_BLACK_DEPTH),
            );
        }
    }

    for i in 0..12 {
        let x = -0.5 * base_width + BASE_MARGIN + KEY_WIDTH + (i as f32) * (KEY_WIDTH + KEY_GAP);
        let note = make_note(settings.note_on_q, i, false).expect("Failed getting white note");
        new_cube(
            note,
            KeyboardRegister::Upper,
            &mut commands,
            materials.ivory.clone(),
            &mut mesh_assets,
            Vec3::new(
                x,
                0.5 * KEY_HEIGHT + ROW_ELEVATION,
                -BASE_MARGIN - 0.5 * KEY_DEPTH - ROW_OFFSET,
            ),
            Vec3::new(KEY_WIDTH, KEY_HEIGHT, KEY_DEPTH),
        );
        if let Some(note) = make_note(settings.note_on_q, i, true) {
            new_cube(
                note,
                KeyboardRegister::Upper,
                &mut commands,
                materials.ebony.clone(),
                &mut mesh_assets,
                Vec3::new(
                    x + 0.5 * (KEY_WIDTH + KEY_GAP),
                    1.5 * KEY_HEIGHT + ROW_ELEVATION,
                    -BASE_MARGIN - KEY_DEPTH + 0.5 * KEY_BLACK_DEPTH - ROW_OFFSET,
                ),
                Vec3::new(KEY_WIDTH, KEY_HEIGHT, KEY_BLACK_DEPTH),
            );
        }
    }
}

fn new_cube(
    note: u8,
    register: KeyboardRegister,
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    position: Vec3,
    size: Vec3,
) {
    commands.spawn((
        KeyWithNote { note, register },
        Mesh3d(mesh_assets.add(Cuboid::from_size(size))),
        MeshMaterial3d(material),
        Transform::from_translation(position),
    ));
}

fn highlight_key_events(
    mut events: EventReader<KeyEvent>,
    materials: Res<PianoMaterials>,
    mut key_query: Query<(
        &KeyWithNote,
        &mut MeshMaterial3d<StandardMaterial>,
        &mut Transform,
    )>,
) {
    for event in events.read() {
        let (note, is_note_on) = match event.message {
            Message {
                data: Event::NoteOn { note, .. },
                ..
            } => (note, true),
            Message {
                data: Event::NoteOff { note, .. },
                ..
            } => (note, false),
            _ => {
                continue;
            }
        };
        if let Some((_, mut material, mut transform)) = key_query
            .iter_mut()
            .find(|k| k.0.note == note && k.0.register == event.register)
        {
            if is_note_on {
                material.0 = materials.illuminated.clone();
                transform.translation.y -= KEY_DEPRESSION;
            } else {
                material.0 = match note_is_black(note) {
                    true => materials.ebony.clone(),
                    false => materials.ivory.clone(),
                };
                transform.translation.y += KEY_DEPRESSION;
            }
        }
    }
}
