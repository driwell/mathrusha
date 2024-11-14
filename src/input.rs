use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use rand::Rng;
use std::mem;

pub struct Input;

impl Plugin for Input {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_keyboard_input_events);
        app.add_systems(Update, update_prompt);
    }
}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<crate::ui::Reply>>,
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

fn update_prompt(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<crate::ui::Prompt>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let number = rand::thread_rng().gen_range(1..=99);
                edit_text.single_mut().sections[0].value = number.to_string();
            }
            _ => continue,
        }
    }
}
