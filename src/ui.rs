use std::mem;

use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub struct Prompt;

#[derive(Component)]
struct Reply;

impl Plugin for Prompt {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_layout);
        app.add_systems(Update, listen_keyboard_input_events);
    }
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2dBundle::default());
    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    "prompt".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 100.0,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            });

            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        "".to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    ..default()
                },
                Reply,
            ));
        });
}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<Reply>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let mut text = edit_text.single_mut();
                if text.sections[0].value.is_empty() {
                    continue;
                }
                let old_value = mem::take(&mut text.sections[0].value);
                println!("{old_value}");
            }

            Key::Space => {
                edit_text.single_mut().sections[0].value.push(' ');
            }
            Key::Backspace => {
                edit_text.single_mut().sections[0].value.pop();
            }
            Key::Character(character) => {
                edit_text.single_mut().sections[0].value.push_str(character);
            }
            _ => continue,
        }
    }
}
